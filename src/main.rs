use std::{fs::write, path::Path, sync::atomic::{AtomicBool, Ordering}};
use clap::Parser;

use rocket::{fs::TempFile, http::Status, request::{FromRequest, Outcome}, Config, Request, State, form::Form};
#[macro_use] extern crate rocket;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Overwrite existing webparty save with starter page
    #[arg(long, default_value_t = false)]
    force: bool,

    /// Enable Authentication
    #[arg(short, long, default_value_t = false)]
    auth: bool,

    /// Use a custom token for auth (requires --auth)
    #[arg(short, long, default_value = None)]
    token: Option<String>,

    /// Path to persist webparty
    #[arg(long, default_value_t = String::from("./webparty.html"))]
    path: String,

    /// Disable checking for webparty client code when writing to persisted file (not recommended)
    #[arg(long, default_value_t = false)]
    disable_check: bool,

    /// Verbose output [enable logging for all requests]
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    
    /// Port to run webparty on
    #[arg(short, long, default_value_t = 8000)]
    port: u16,

    /// Disable image uploading
    // TODO
    #[arg(short, long, default_value_t = false)]
    disable_images: bool,

    /// Folder to persist images to
    #[arg(long, default_value_t = String::from("./assets"))]
    assets_path: String,
}

struct Token<'r>(&'r str);

#[derive(Debug)]
enum TokenError {
    Missing,
    Invalid
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = TokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        // We have to retrieve the state from the rocket object
        let state = req.guard::<&State<PartyOptions>>().await.unwrap();

        if !state.auth.load(Ordering::Relaxed) {
            // Auth is disabled so return success to return to request handler
            return Outcome::Success(Token(""));
        } else {
            // Get token value from managed state
            let token = state.token.as_ref().unwrap();

            // Match the Auth header. Return Error outcome to fail the request immediately
            match req.headers().get_one("Authorization") {
                None => Outcome::Error((Status::BadRequest, TokenError::Missing)),
                Some(key) if key == token => Outcome::Success(Token(key)),
                Some(_) => Outcome::Error((Status::BadRequest, TokenError::Invalid)),
            }
        }
    }
}

#[derive(FromForm)]
struct Upload<'r> {
    file: TempFile<'r>
}

#[derive(Responder)]
#[response(status = 200, content_type = "html")]
struct PartyHtml(String);

#[derive(Responder)]
#[response(status = 200, content_type = "text/javascript")]
struct DeveloperJS(String);

#[derive(Responder)]
#[response(status = 200, content_type = "text/javascript")]
struct StaticJS(&'static [u8]);

#[derive(Responder)]
#[response(status = 500, content_type = "html")]
struct StaticPartyError(&'static [u8]);




// Include webparty.js and default html as bytes
static PARTYJS: &'static [u8] = include_bytes!("webparty.js");
static PARTYHTML: &'static [u8] = include_bytes!("default.html");
static PARTYERRORHTML: &'static [u8] = include_bytes!("error.html");



/// Options that should be exposed to route handlers
struct PartyOptions {
    // Authentication
    auth: AtomicBool,
    token: Option<String>,

    // Path
    path: String,

    // Disable checking for webparty.js tag
    disable_check: bool,

    // Path to persist images
    assets_path: String
}

#[catch(500)]
fn internal_error(req: &Request) -> StaticPartyError {
    println!("Error: {:?}", req);
    StaticPartyError(PARTYERRORHTML)
}


// Serve the main page
#[get("/")]
fn index(state: &State<PartyOptions>) -> PartyHtml {
    let html = std::fs::read_to_string(&state.path).unwrap();
    PartyHtml(html)
}

#[get("/Dont_remove_this")]
fn webparty() -> DeveloperJS {
    let js = std::fs::read_to_string("src/webparty.js").unwrap();
    DeveloperJS(js)
}

#[put("/update", data="<markup>")]
async fn push_html(markup: String, state: &State<PartyOptions>, _auth: Token<'_>) -> Status {

    // Check for client code if checks aren't disabled
    if !state.disable_check && !markup.contains(r#"<script src="/Dont_remove_this"></script>"#) {
       return Status::BadRequest
    }

    // Auth is handled by request guard https://api.rocket.rs/v0.5/rocket/request/trait.FromRequest.html#outcomes
    write(&state.path, markup).unwrap();
    Status::Ok
}

#[post("/upload", data="<upload>")]
async fn upload_file(mut upload: Form<Upload<'_>>, state: &State<PartyOptions>) -> Status {
    upload.file.persist_to(&state.assets_path).await.unwrap(); 
    return Status::Ok
}



#[launch]
fn rocket() -> _ {
    let args = Args::parse();
    
    println!("Starting webparty on port {}...", args.port);

    if !args.verbose {
        println!("Start with -v to enable request logging");
    }

    if !Path::new(&args.path).exists() || args.force {
        std::fs::write(&args.path, PARTYHTML).unwrap();
    }

    let token = if args.auth && args.token.is_none() {
        println!("Looks like you didn't provide a secret password with --token <TOKEN>. I'll generate one for you.");

        let token = rand::random::<u64>().to_string();
        println!("Your token is: {}", token);
        println!();

        Some(token)
    } else if args.auth && args.token.is_some() {
        Some(args.token.clone().unwrap())
    } else {
        None
    };

    // Managed state to be accessed by route handlers
    let options = PartyOptions {
        auth: AtomicBool::new(args.auth),
        token: token,
        path: args.path,
        disable_check: args.disable_check,
        assets_path: args.assets_path
    };
    

    // Sever config
    let config = Config {
        port: args.port,
        log_level: if args.verbose { rocket::config::LogLevel::Normal } else { rocket::config::LogLevel::Off },
        ..Config::default()
    };


    rocket::build()
        .configure(config)
        .register("/", catchers![internal_error])
        .mount("/", routes![index, webparty, push_html, upload_file])
        .manage(options)
}

#[cfg(test)] mod tests;