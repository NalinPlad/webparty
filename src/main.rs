use std::{fs::write, path::Path, sync::atomic::AtomicBool};

use clap::Parser;
use rocket::{http::Status, request::{self, FromRequest, Outcome}, Request, State};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Overwrite existing webparty save with starter page
    #[arg(short, long, default_value_t = false)]
    force: bool,

    /// Enable Authentication
    #[arg(short, long, default_value_t = false)]
    auth: bool,

    /// Use a custom token for auth (requires --auth)
    #[arg(short, long, default_value = None)]
    token: Option<String>,

    /// Path to persist webparty
    #[arg(short, long, default_value_t = String::from("./webparty.html"))]
    path: String,

    /// Disable checking for webparty client code when writing to persisted file (not recommended)
    #[arg(short, long, default_value_t = false)]
    disable_check: bool,

    /// Verbose output
    // TODO
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Port to run webparty on
    // TODO
    #[arg(short, long, default_value_t = 8000)]
    port: u16
}

#[macro_use] extern crate rocket;


#[derive(Responder)]
#[response(status = 200, content_type = "html")]
struct PartyHtml(String);

#[derive(Responder)]
#[response(status = 200, content_type = "text/javascript")]
struct StaticJS(&'static [u8]);

// Include webparty.js and default html as bytes
static PARTYJS: &'static [u8] = include_bytes!("webparty.js");
static PARTYHTML: &'static [u8] = include_bytes!("default.html");


// Options that should be exposed to route handlers
struct PartyOptions {
    // Authentication
    auth: AtomicBool,
    token: Option<String>,

    // Path
    path: String,

    // Disable checking for webparty.js tag
    disable_check: bool
}


// Serve the main page
#[get("/")]
fn index() -> PartyHtml {
    let html = std::fs::read_to_string("./webparty.html").unwrap();
    PartyHtml(html)
}

#[get("/Dont remove this")]
fn webparty() -> StaticJS {
    StaticJS(PARTYJS)
}

struct Token(String);

#[derive(Debug)]
enum AuthError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = req.headers().get("Authorization").collect();

        if keys.len() != 1 {
            return Outcome::Error((Status::Unauthorized, AuthError::Missing));
        }

        let key = keys[0];

        if key != "Basic " {
            return Outcome::Error((Status::Unauthorized, AuthError::Invalid));
        }
        Outcome::Success(Token(key.to_string()))
    }
}


#[put("/update", data="<markup>")]
async fn push_html(markup: String, state: &State<PartyOptions>) -> &'static str {
    if !state.disable_check && !markup.contains(r#"<script src="/Dont remove this"></script>"#) {
        return "ERROR: webparty.js tag not found in markup";
    }

    
    write("./webparty.html", markup).unwrap();
    "ACK"
}



#[launch]
fn rocket() -> _ {
    let args = Args::parse();

    if !Path::new(&args.path).exists() || args.force {
        std::fs::write(&args.path, PARTYHTML).unwrap();
    }

    let options = PartyOptions {
        auth: AtomicBool::new(args.auth),
        token: args.token,
        path: args.path,
        disable_check: args.disable_check
    };

    rocket::build()
    .mount("/", routes![index, webparty, push_html])
    .manage(options)
}