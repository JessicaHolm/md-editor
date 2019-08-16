// Copyright (c) 2019 Jason Holm
// This program is licensed under the "MIT License". Please
// see the file `LICENSE` for license terms.

// Code adapted from the yew examples page
// https://github.com/yewstack/yew/tree/master/examples/two_apps

mod markdown;

use markdown::*;

use yew::html::Scope;
use yew::App;

use stdweb::web::{document, IParentNode};

fn mount_app(selector: &'static str, app: App<Model>) -> Scope<Model> {
    let element = document().query_selector(selector).unwrap().unwrap();
    app.mount(element)
}

fn main() {
    yew::initialize();
    let markdown = App::new();
    let html = App::new();
    let mut to_markdown = mount_app(".markdown", markdown);
    let mut to_html = mount_app(".html", html);
    to_markdown.send_message(Msg::SetScope(to_html.clone()));
    to_html.send_message(Msg::SetScope(to_markdown.clone()));
    yew::run_loop();
}
