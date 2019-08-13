//use std::convert::TryInto;
//use std::convert::From;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::html::Scope;

use stdweb::js;
use stdweb::traits::*;
use stdweb::web::document;
use stdweb::web::html_element::TextAreaElement;
use stdweb::Value;
use stdweb::Value::Null;
use stdweb::unstable::TryFrom;

use comrak::{markdown_to_html, ComrakOptions};

pub const INITAL_TEXT: &str = "# Hello World\nThis is a simple markdown editor written in Rust. It support standard Markdown features such as:\n\n **bold** and *italic* text.\n\n* Bullet points\n* Another bullet point\n1. Numbered lists\n2. Second item\n3. Third item\n\nCode blocks are also supported.\n\n```\nfn main() {\n    println!(\"Hello world\");\n}\n```";

//pub struct MyTextArea(TextAreaElement);

pub struct Model {
    scope: Option<Scope<Model>>,
    value: String,
    area: Value,
}

pub enum Msg {
    SetScope(Scope<Model>),
    GotInput(String),
    SetText(String),
    Clicked,
}

/*
impl From<Element> for MyTextArea {
    fn from(value: Element) -> Self {
        MyTextArea(value)
    }
}
*/

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { 
            scope: None,
            value: "".to_string(),
            area: Null,
        }
    }

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
                self.area = js! { document.getElementById("area") };
            }
            Msg::SetText(new_value) => {
                self.value = new_value;
            }
            Msg::Clicked => {
                js! { document.getElementById("upload").click() };
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let html = html! {
            <html>
            <div>
            <form>
                <img src="https://icon-library.net/images/file-upload-icon/file-upload-icon-18.jpg" onclick=|_| Msg::Clicked height="32" width="32" style="cursor:pointer" />
                <input type="file" id="upload" style="display:none"/>
            </form>
            </div>
            <div>
                <textarea rows=30 cols=100 id="area" value=&self.value oninput=|e| Msg::GotInput(e.value) placeholder="Type markdown here.">
                </textarea>
            </div>
            </html>
        };
        
        
        
        let v = self.area.clone();
        let x = TextAreaElement::try_from(v).unwrap(); //broken line
        //let y = x.value();
        //if &y != &self.value {
            render(&self.value);
            display_info(&self.value);
        //}
        html
    }
}

fn calculate_info(value: &str) -> String {
    let w: usize = value.split_whitespace().count();
    let c: usize = value.chars().count();
    let l: usize = value.lines().count();
    format!("{} characters, {} words, {} lines",c, w, l)
}

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
