#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use comrak::{markdown_to_html, ComrakOptions};
use horrorshow::helper::doctype;
use horrorshow::{html, Raw};
use rocket::http::RawStr;
use rocket::request::Form;
use rocket::response::{NamedFile, Redirect};

#[derive(FromForm)]
struct UserInput {
    input: String,
}

impl UserInput {
    fn new(text: &str) -> UserInput {
        let input = text.to_string();
        UserInput { input }
    }

    fn render(&self, text: &str) -> String {
        format!(
            "{}",
            html!(
                : doctype::HTML;
                html {
                    body {
                        : Raw(markdown_to_html(text, &ComrakOptions::default()))
                    }
                }
            )
        )
    }
}

#[post("/", data = "<text>")]
fn markdown(text: Form<UserInput>) -> Result<Redirect, File> {
    let user = UserInput::new(text.input.as_str());
    let rendered = user.render(&user.input);

    let mut file = File::create(Path::new("display")).expect("Error");
    file.write_all(rendered.as_bytes()).expect("Error");
    Ok(Redirect::to("display"))
}

#[get("/<filename>")]
fn display(filename: &RawStr) -> Result<File, std::io::Error> {
    File::open(filename.as_str())
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, markdown, display])
}

fn main() {
    rocket().launch();
}
