use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::html::Scope;

use stdweb::traits::*;
use stdweb::web::document;

use comrak::{markdown_to_html, ComrakOptions};

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

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { 
            scope: None,
            value: "".into(),
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
            }
            Msg::SetText(new_value) => {
                self.value = new_value;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        render(&self.value);
        display_info(&self.value);
        html! {
            <html>
            <div>
                <textarea rows=30 cols=100
                value=&self.value
                oninput=|e| Msg::GotInput(e.value)
                placeholder="Type markdown here.">
                </textarea>
            </div>
            </html>
        }
    }
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
    let node = document().get_element_by_id("first").unwrap();
    let info = format!("{} chars", value.len());
    node.append_html(&info).unwrap();
}
