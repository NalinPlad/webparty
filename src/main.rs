use rocket::{data::Capped, response::content::{RawHtml, RawJavaScript, RawText}};

#[macro_use] extern crate rocket;


#[derive(Responder)]
#[response(status = 200, content_type = "html")]
struct PartyHtml(String);

#[derive(Responder)]
#[response(status = 200, content_type = "text/javascript")]
struct StaticJS(&'static [u8]);

// Include webparty.js as bytes
static PARTYJS: &'static [u8] = include_bytes!("webparty.js");
static PARTYHTML: &'static [u8] = include_bytes!("webparty.html");

#[get("/")]
fn index() -> PartyHtml {
    let html = std::fs::read_to_string("./webparty.html").unwrap();
    PartyHtml(html)
}

#[get("/Dont🫥Remove🫥This🫥Line!")]
fn webparty() -> StaticJS {
    StaticJS(PARTYJS)
}

#[post("/update", data="<markup>")]
async fn push_html(markup: String) -> &'static str {
    
    std::fs::write("./webparty.html", markup).unwrap();
    "ACK"
}

#[launch]
fn rocket() -> _ {
    std::fs::write("./webparty.html", PARTYHTML).unwrap();
    rocket::build().mount("/", routes![index, push_html, webparty])
}