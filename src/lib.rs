use comrak::{markdown_to_html, ComrakOptions};
//use stdweb::js;
use stdweb::web::document;
use stdweb::traits::*;
//use yew::prelude::*;
use yew::html::Scope;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    scope: Option<Scope<Model>>,
    //selector: &'static str,
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
            //selector: "",
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
        let html = html! {
            <html>
            <div>
                <textarea rows=20 cols=70
                value=&self.value
                oninput=|e| Msg::GotInput(e.value)
                placeholder="Type markdown here.">
                </textarea>
            </div>
            </html>
        };
        
        let node = document().get_element_by_id("second").unwrap();
        let parent_node = node.parent_node().unwrap();
        let new_node = document().create_element("div").unwrap();
        let md = &markdown_to_html(&self.value, &ComrakOptions::default());
        //node.set_node_value(Some(""));
        let last = match &self.value.chars().last() {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };
        //node.append_html(&last);
        new_node.set_attribute("id", "second");
        new_node.set_attribute("class", "html");
        parent_node.replace_child(&new_node, &node);
        
        new_node.append_html(md);
        //document().body().unwrap().append_child(&node);
        
        html
    }
}
