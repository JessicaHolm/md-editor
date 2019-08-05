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
            value: "Nothing".into(),
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
            <div>
                <textarea rows=20 cols=70
                value=&self.value
                oninput=|e| Msg::GotInput(e.value)
                placeholder="Type markdown here.">
                </textarea>
            </div>
        };
        
        let node = document().get_element_by_id("second");
        let x = match node {
            Some(_) => "Hello",
            None => "World",
        };
        //let node = document().get_element_by_id("second").unwrap();
        let parent_node = node.unwrap().parent_node().unwrap();
        let new_node = document().create_element("div").unwrap();
        //let md = markdown_to_html(&self.value, &ComrakOptions::default());
        new_node.append_html(x);
        parent_node.replace_child(&new_node, &node);
        
        //document().body().unwrap().append_child(&new_node);
        
        html
    }
}
