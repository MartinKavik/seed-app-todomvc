#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use std::collections::BTreeMap;
use ulid::Ulid;

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
    todos: BTreeMap<Ulid, Todo>,
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
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("UrlChanged", url);
        }
        Msg::NewTodoTitleChanged(title) => {
            log!("NewTodoTitleChanged", title);
        }
     
        // ------ Basic Todo operations ------
        Msg::CreateTodo => {
            log!("CreateTodo");
        }
        Msg::ToggleTodo(id) => {
            log!("ToggleTodo");
        }
        Msg::RemoveTodo(id) => {
            log!("RemoveTodo");
        }
        
        // ------ Bulk operations ------
        Msg::CheckOrUncheckAll => {
            log!("CheckOrUncheckAll");
        }
        Msg::ClearCompleted => {
            log!("ClearCompleted");
        }
        
        // ------ Selection ------
        Msg::SelectTodo(opt_id) => {
            log!("SelectTodo", opt_id);
        },
        Msg::SelectedTodoTitleChanged(title) => {
            log!("SelectedTodoTitleChanged", title);
        },
        Msg::SaveSelectedTodo => {
            log!("SaveSelectedTodo");
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        "I'm a placeholder"
      ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
