#[macro_use]
extern crate yew;

use std::fmt;
use yew::prelude::*;

type Context = ();

#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}
impl fmt::Display for Todo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let done = if self.done { "X" } else { " " };
        write!(fmt, "[{}] - {}", done, self.title)
    }
}

#[derive(Debug)]
struct Model {
    currentId: u32,
    input: String,
    todos: Vec<Todo>,
}

// TODO: Add update
enum Msg {
    UpdateInput(String),
    Create,
    Delete(u32),
}

impl Component<Context> for Model {
    // Some details omitted. Explore the examples to get more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            currentId: 0,
            input: String::new(),
            todos: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::UpdateInput(value) => {
                self.input = value;
                true
            }
            Msg::Create => {
                let todo = Todo {
                    id: self.currentId,
                    title: self.input.clone(),
                    done: false,
                };
                self.todos.push(todo);
                self.currentId = self.currentId + 1;
                self.input = String::new();
                true
            }
            Msg::Delete(id) => {
                self.todos.retain(|todo| todo.id != id);
                true
            }
        }
    }
}

impl Renderable<Context, Model> for Model {

    fn view(&self) -> Html<Context, Self> {
        let todo_view = |todo: &Todo| {
            let id = todo.id;
            html! {
                <li>
                    <span>{&todo.title}</span>
                    <button onclick=|_| Msg::Delete(id),>{"X"}</button>
                </li>
            }
        };
        html! {
            // Render your model here
            <div>
                <input
                    placeholder="What do you have to do?",
                    value=&self.input,
                    oninput=|e| Msg::UpdateInput(e.value), />
            </div>
            <button onclick=|_| Msg::Create,>{ "Click me!" }</button>
            <div>
                <ul>
                    {for self.todos.iter().map(todo_view)}
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
