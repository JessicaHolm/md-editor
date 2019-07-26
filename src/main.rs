#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io;

use rocket::response::Redirect;
use rocket::response::NamedFile;
use rocket::request::{Form, FromFormValue};
use rocket::http::RawStr;
use comrak::{markdown_to_html, ComrakOptions};
use horrorshow::Raw;
use horrorshow::helper::doctype;
use horrorshow::html;

#[derive(Debug)]
struct ValidInput<'a>(&'a str);

#[derive(FromForm)]
struct UserInput<'a> {
    input: Result<ValidInput<'a>, &'static str>,
}

impl<'v> FromFormValue<'v> for ValidInput<'v> {
    type Error = &'static str;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        if v.len() == 0 {
            Err("too short!")
        } else {
            Ok(ValidInput(v.as_str()))
        }
    }
}

#[post("/markdown", data = "<text>")]
fn markdown(text: Form<UserInput>) -> Result<Redirect, String> {
    Ok(UserInput::render(format!("{}", UserInput::input)))
}

impl<'a> UserInput<'a> {
    fn render(text: &RawStr) -> String {
        format!("{}", html!(
            : doctype::HTML;
            html {
                body {
                    : Raw(markdown_to_html("# Hello", &ComrakOptions::default()))
                }
            }
        )
        )
    }
}

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, markdown])
}

fn main() {
    rocket().launch();
}
