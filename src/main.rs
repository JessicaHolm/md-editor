use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use comrak::{markdown_to_html, ComrakOptions};

pub struct Model {
    value: String,
}

pub enum Msg {
    GotInput(String),
    Clicked,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { value: "".into() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.value = new_value;
            }
            Msg::Clicked => {
                self.value = "blah blah blah".to_string();
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <textarea rows=20
                        cols=70
                        value=&self.value
                        oninput=|e| Msg::GotInput(e.value)
                        placeholder="Type markdown here.">
                    </textarea>
                </div>
                <div>
                    <p>
                    {markdown_to_html(&self.value, &ComrakOptions::default())}
                    </p>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
