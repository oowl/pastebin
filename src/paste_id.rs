use std::fmt;
use std::borrow::Cow;
use rand::{self,Rng};
use rocket::request::{self,FromParam,FromRequest,Request};
use rocket::http::RawStr;
use rocket::outcome::Outcome::*;
#[derive(Debug)]
pub struct UserAgent(String);

impl fmt::Display for UserAgent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserAgent {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let us = request.headers().get_one("User-Agent").unwrap();
        let a = us.contains("Mozilla");
        if a {
            Forward(())
        } else {
            Success(UserAgent("sss".to_string()))
        }
    }
}



const BASE62: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub struct PasteId<'a>(Cow<'a,str>);
impl<'a> PasteId<'a> {
    pub fn new(size: usize) -> PasteId<'static> {
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }
        PasteId(Cow::Owned(id))
    }
}

impl<'a> fmt::Display for PasteId<'a> {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.0)
    }
}

fn valid_id(id: &str) -> bool {
    id.chars().all(|c| {
        (c >= 'a' && c <= 'z')
            || (c >= 'A' && c <= 'Z')
            || (c >= '0' && c <= '9')
    })
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a RawStr;
    fn from_param(param: &'a RawStr) -> Result<PasteId<'a>,&'a RawStr> {
        match valid_id(param) {
            true => Ok(PasteId(Cow::Borrowed(param))),
            false => Err(param)
        }
    }
}