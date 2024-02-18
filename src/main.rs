use std::path::Path;

use clap::Parser;
use rocket::request::FromRequest;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Wether to overwrite webparty file with default contents
    #[arg(short, long, default_value_t = false)]
    force: bool,

    /// Wether to enable authentication
    #[arg(short, long, default_value_t = false)]
    auth: bool,

    /// Use a custom token for auth
    #[arg(short, long, default_value = None)]
    token: Option<String>,

    /// Path to use to persist webparty
    #[arg(short, long, default_value_t = String::from("./webparty.html"))]
    path: String
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

#[get("/")]
fn index() -> PartyHtml {
    let html = std::fs::read_to_string("./webparty.html").unwrap();
    PartyHtml(html)
}

#[get("/Dont remove this")]
fn webparty() -> StaticJS {
    StaticJS(PARTYJS)
}

#[put("/update", data="<markup>")]
async fn push_html(markup: String) -> &'static str {
    std::fs::write("./webparty.html", markup).unwrap();
    "ACK"
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse();
    // check if ./webparty.html exists
    if !Path::new(&args.path).exists() || args.force {
        std::fs::write(&args.path, PARTYHTML).unwrap();
    }

    if args.auth {
        if args.token.is_none() {
            
        }
    }

    rocket::build().mount("/", routes![index, push_html, webparty])
}