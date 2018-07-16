#[macro_use] extern crate yew;
use yew::html::*;
use yew::prelude::*;

type Context = ();

struct Model {
    input: String,
    todos: Vec<String>,
}

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    RemoveAll,
    Nothing,
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model { input: String::from(""), todos: vec![] }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Add => false,
            Msg::Update(s) => false,
            Msg::Remove(i) => false,
            Msg::RemoveAll => false,
            Msg::Nothing => false,
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <h1>{"Todo App"}</h1>
            </div>
            <div>
                <button>{"Delete all todos!"}</button>
            </div>
            <div>
                <ul>
                </ul>
            </div>
        }
    }
}


fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
