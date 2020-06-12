#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

struct Model {
    todos: BtreeMap<Ulid, Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>,
    filter: Filter,
    base_url: Url,
}

struct Todo {
    id: Ulid,
    title: String,
    completed: bool,
    element: ElRef<web_sys::HtmlElement>,
}

struct SelectedTodo {
    id: Ulid,
    title: String,
}

enum Filter {
   All,
   Active,
   Completed,
}

// ------ ------
//    Update
// ------ ------

#[derive(Copy, Clone)]
enum Msg {
    UrlChanged(subs::UrlChanged),
    NewTodoTitleChanged(String),
 
    // ------ Basic Todo operations ------
    CreateTodo,
    ToggleTodo(Ulid),
    RemoveTodo(Ulid),
    
    // ------ Bulk operations ------
    CheckOrUncheckAll,
    ClearCompleted,
    
    // ------ Selection ------
    SelectTodo(Option<Ulid>),
    SelectedTodoTitleChanged(String),
    SaveSelectedTodo,
 }

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

// ------ ------
//     View
// ------ ------

#[allow(clippy::trivially_copy_pass_by_ref)]
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
