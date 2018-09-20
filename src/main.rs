#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rand;
mod paste_id;
use self::paste_id::PasteId;
use self::paste_id::UserAgent;
extern crate rocket;
extern crate rocket_contrib;

use std::io;
use std::path::Path;
use rocket::Data;
use rocket::http::RawStr;
use std::fs::File;
use rocket::request::FromParam;
use rocket_contrib::Template;
use rocket::response::Redirect;
#[macro_use] 
extern crate serde_derive;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

enum Index {
    string(String),
    tem(Template),
}
#[get("/")]
fn index(ua: UserAgent) -> &'static str {
        "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    \n"
}

#[get("/",rank=2)]
fn index_tem() -> Template {
    let context: TemplateContext = TemplateContext{ name: "hello world".to_string() };
    Template::render("index", &context)
}

#[post("/",data="<paste>")]
fn upload(paste: Data) -> io::Result<String> {
    let id = PasteId::new(3);
    let filename = format!("upload/{id}",id = id);
    let url = format!("{host}/{id}\n",host = "http://localhost:8000", id = id);
    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: PasteId) -> Option<File> {
    let filename = format!("upload/{id}",id = id);
    File::open(&filename).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index,upload,retrieve,index_tem]).attach(Template::fairing()).launch();
}
