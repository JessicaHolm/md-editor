// This program is licensed under the "MIT License". Please
// see the file `LICENSE` for license terms.

// Code adapted from the yew examples page
// https://github.com/yewstack/yew/blob/master/examples/textarea/src/lib.rs

use yew::html::Scope;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use stdweb::traits::*;
use stdweb::web::document;

use comrak::{markdown_to_html, ComrakOptions};

pub const INITAL_TEXT: &str = "# Hello World\n---\n\nThis is a simple Markdown editor written in Rust. It supports standard Markdown features such as:\n\n **bold** and *italic* text.\n\n* Bullet points\n* Another bullet point\n1. Numbered lists\n2. Second item\n3. Third item\n\nCode blocks are also supported.\n\n```\nfn main() {\n    println!(\"Hello world\");\n}\n```\n\nLinks are supported as well [this](https://github.com/JasonHolm/md-editor) is the link to this project's GitHub page.\n\nLastly, you can embed images. Here is the GitHub logo.\n\n![Github Logo](https://cdn.iconscout.com/icon/free/png-256/github-153-675523.png)";

pub struct Model {
    scope: Option<Scope<Model>>,
    value: String,
}

pub enum Msg {
    SetScope(Scope<Model>),
    GotInput(String),
    SetText(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    // Create a new Model with initialized values.
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            scope: None,
            value: INITAL_TEXT.to_string(),
        }
    }

    // Whenever something changes update the Model's fields.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetScope(scope) => {
                self.scope = Some(scope);
            }
            Msg::GotInput(text) => {
                self.scope
                    .as_mut()
                    .unwrap()
                    .send_message(Msg::SetText(text));
            }
            Msg::SetText(new_value) => {
                self.value = new_value;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    // Redraw the screen after a change occurs.
    fn view(&self) -> Html<Self> {
        render(&self.value);
        display_info(&self.value);
        html! {
            <html>
            <div>
                <textarea value=&self.value oninput=|e| Msg::GotInput(e.value) placeholder="Type markdown here.">
                </textarea>
            </div>
            </html>
        }
    }
}

// Count the number of characters, words, and lines.
fn calculate_info(value: &str) -> String {
    let c: usize = value.chars().count();
    let w: usize = value.split_whitespace().count();
    let l: usize = value.lines().count();
    format!("{} characters, {} words, {} lines", c, w, l)
}

// Render the plain text on the left to Markdown on the right.
fn render(value: &str) {
    let node = document().get_element_by_id("second").unwrap();
    let parent_node = node.parent_node().unwrap();
    let new_node = document().create_element("div").unwrap();
    let md = &markdown_to_html(value, &ComrakOptions::default());
    new_node.set_attribute("id", "second").unwrap();
    new_node.set_attribute("class", "html split right").unwrap();
    parent_node.replace_child(&new_node, &node).unwrap();
    new_node.append_html(md).unwrap();
}

// Display the info bar at the bottom of the screen.
fn display_info(value: &str) {
    let node = document().get_element_by_id("third").unwrap();
    let parent_node = node.parent_node().unwrap();
    let new_node = document().create_element("div").unwrap();
    let info = &calculate_info(value);
    new_node.set_attribute("id", "third").unwrap();
    new_node.set_attribute("class", "bottom").unwrap();
    parent_node.replace_child(&new_node, &node).unwrap();
    new_node.append_html(info).unwrap();
}
