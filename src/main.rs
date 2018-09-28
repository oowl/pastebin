#![feature(plugin,decl_macro)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
extern crate rand;
mod paste_id;
use self::paste_id::PasteId;
use self::paste_id::UserAgent;
extern crate rocket;
extern crate rocket_contrib;

use std::io;
use std::path::{Path, PathBuf};
use rocket::Data;
use rocket::http::RawStr;
use std::fs::File;
use std::io::prelude::*;
use rocket::request::FromParam;
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::response::NamedFile;
use rocket::request::Form;
#[macro_use] 
extern crate serde_derive;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
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
fn upload(paste: Data,ua: UserAgent) -> io::Result<String> {
    let id = PasteId::new(3);
    let filename = format!("upload/{id}",id = id);
    let url = format!("/{id}\n", id = id);
    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}
#[derive(FromForm)]
struct Paste {
    content: String,
}
#[post("/",data="<paste>",rank = 2)]
fn upload_red(paste: Form<Paste>) -> io::Result<Redirect> {
    let id = PasteId::new(3);
    let filename = format!("upload/{id}",id = id);
    let url = format!("/{id}\n", id = id);
    if paste.get().content == ""{
        return Ok(Redirect::to("/"))
    }
    let mut file = File::create(filename)?;
    file.write_all(paste.get().content.as_bytes())?;
    Ok(Redirect::to(&url))
}

#[get("/<id>")]
fn retrieve(id: PasteId,ua: UserAgent) -> Option<File> {
    let filename = format!("upload/{id}",id = id);
    File::open(&filename).ok()
}

#[get("/<id>",rank=2)]
fn retrieve_tem(id: PasteId) -> Option<Template> {
    let filename = format!("upload/{id}",id = id);
    let mut s = String::new();
    let f = File::open(&filename);
    match f {
        Ok(mut f) => {
            f.read_to_string(&mut s).ok();
            let context: TemplateContext = TemplateContext{ name: s };
            Some(Template::render("id", &context))
        }
        Err(_) => None
    }
}
#[get("/<file..>",rank=3)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/").join(file)).ok()
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index,upload,retrieve,retrieve_tem,index_tem,upload_red,files]).attach(Template::fairing()).launch();
}
