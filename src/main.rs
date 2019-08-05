use stdweb::web::{document, IParentNode};
use mdeditor::{Model, Msg};
use yew::html::Scope;
use yew::App;

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
