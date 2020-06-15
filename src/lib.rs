#![allow(clippy::wildcard_imports)]
// TODO: Remove
#![allow(dead_code, unused_variables)]

use seed::{prelude::*, *};

use std::collections::BTreeMap;
use std::mem;
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use ulid::Ulid;

const ENTER_KEY: &str = "Enter";
const ESCAPE_KEY: &str = "Escape";

const STORAGE_KEY: &str = "todos-seed";

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        todos: LocalStorage::get(STORAGE_KEY).unwrap_or_default(),
        new_todo_title: String::new(),
        selected_todo: None,
        filter: Filter::All,
        base_url: Url::new(),
    }
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

#[derive(Deserialize, Serialize)]
struct Todo {
    id: Ulid,
    title: String,
    completed: bool,
}

struct SelectedTodo {
    id: Ulid,
    title: String,
    input_element: ElRef<web_sys::HtmlInputElement>,
}

#[derive(Copy, Clone, Eq, PartialEq, EnumIter)]
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

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("UrlChanged", url);
        }
        Msg::NewTodoTitleChanged(title) => {
            model.new_todo_title = title;
        }
     
        // ------ Basic Todo operations ------

        Msg::CreateTodo => {
            let title = model.new_todo_title.trim();
            if not(title.is_empty()) {
                let id = Ulid::new();
                model.todos.insert(id, Todo {
                    id,
                    title: title.to_owned(),
                    completed: false,
                });
                model.new_todo_title.clear();
            }
        }
        Msg::ToggleTodo(id) => {
            if let Some(todo) = model.todos.get_mut(&id) {
                todo.completed = not(todo.completed);
            }
        }
        Msg::RemoveTodo(id) => {
            model.todos.remove(&id);
        }
        
        // ------ Bulk operations ------

        Msg::CheckOrUncheckAll => {
            let all_checked = model.todos.values().all(|todo| todo.completed);
            for todo in model.todos.values_mut() {
                todo.completed = not(all_checked);
            }
        }
        Msg::ClearCompleted => {
            // TODO: Refactor with `BTreeMap::drain_filter` once stable.
            model.todos = mem::take(&mut model.todos)
                .into_iter()
                .filter(|(_, todo)| not(todo.completed))
                .collect();
        }
        
        // ------ Selection ------

        Msg::SelectTodo(Some(id)) => {
            if let Some(todo) = model.todos.get(&id) {
                let input_element = ElRef::new();
                
                model.selected_todo = Some(SelectedTodo {
                    id,
                    title: todo.title.clone(),
                    input_element: input_element.clone(),
                });

                let title_length = u32::try_from(todo.title.len()).expect("title length as u32");
                orders.after_next_render(move |_| {
                    let input_element = input_element.get().expect("input_element");

                    input_element
                        .focus()
                        .expect("focus input_element");

                    input_element
                        .set_selection_range(title_length, title_length)
                        .expect("move cursor to the end of input_element");
                });
            }
        },
        Msg::SelectTodo(None) => {
            model.selected_todo = None;
        },
        Msg::SelectedTodoTitleChanged(title) => {
            if let Some(selected_todo) = &mut model.selected_todo {
                selected_todo.title = title;
            }
        },
        Msg::SaveSelectedTodo => {
            if let Some(selected_todo) = model.selected_todo.take() {
                if let Some(todo) = model.todos.get_mut(&selected_todo.id) {
                    todo.title = selected_todo.title;
                }
            }
        }
    }
    LocalStorage::insert(STORAGE_KEY, &model.todos).expect("save todos to LocalStorage");
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        view_header(&model.new_todo_title),
        IF!(not(model.todos.is_empty()) => vec![
            view_main(&model.todos, model.selected_todo.as_ref()), 
            view_footer(&model.todos, model.filter),
        ]),
    ]
}

// ------ header ------

fn view_header(new_todo_title: &str) -> Node<Msg> {
    header![C!["header"],
        h1!["todos"],
        input![C!["new-todo"],
            attrs!{
                At::Placeholder => "What needs to be done?", 
                At::AutoFocus => AtValue::None,
                At::Value => new_todo_title,
            },
            input_ev(Ev::Input, Msg::NewTodoTitleChanged),
            keyboard_ev(Ev::KeyDown, |keyboard_event| {
                IF!(keyboard_event.key() == ENTER_KEY => Msg::CreateTodo)
            }),
        ]
    ]
}

// ------ main ------

fn view_main(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg> {
    section![C!["main"],
        view_toggle_all(todos),
        view_todo_list(todos, selected_todo),
    ]
}

fn view_toggle_all(todos: &BTreeMap<Ulid, Todo>) -> Vec<Node<Msg>> {
    let all_completed = todos.values().all(|todo| todo.completed);
    vec![
        input![C!["toggle-all"], 
            attrs!{
                At::Id => "toggle-all", At::Type => "checkbox", At::Checked => all_completed.as_at_value()
            },
            ev(Ev::Change, |_| Msg::CheckOrUncheckAll),
        ],
        label![attrs!{At::For => "toggle-all"}, "Mark all as complete"],
    ]
}

fn view_todo_list(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg> {
    ul![C!["todo-list"],
        todos.values().map(|todo| {
            let id = todo.id;
            let is_selected = Some(id) == selected_todo.map(|selected_todo| selected_todo.id);

            li![C![IF!(todo.completed => "completed"), IF!(is_selected => "editing")],
                el_key(&todo.id),
                div![C!["view"],
                    input![C!["toggle"], 
                        attrs!{At::Type => "checkbox", At::Checked => todo.completed.as_at_value()},
                        ev(Ev::Change, move |_| Msg::ToggleTodo(id)),
                    ],
                    label![
                        &todo.title,
                        ev(Ev::DblClick, move |_| Msg::SelectTodo(Some(id))),
                    ],
                    button![C!["destroy"],
                        ev(Ev::Click, move |_| Msg::RemoveTodo(id))
                    ],
                ],
                IF!(is_selected => {
                    let selected_todo = selected_todo.unwrap();
                    input![C!["edit"], 
                        el_ref(&selected_todo.input_element), 
                        attrs!{At::Value => selected_todo.title},
                        input_ev(Ev::Input, Msg::SelectedTodoTitleChanged),
                        keyboard_ev(Ev::KeyDown, |keyboard_event| {
                            Some(match keyboard_event.key().as_str() {
                                ESCAPE_KEY => Msg::SelectTodo(None),
                                ENTER_KEY => Msg::SaveSelectedTodo,
                                _ => return None
                            })
                        }),
                        ev(Ev::Blur, |_| Msg::SaveSelectedTodo),
                    ]
                }),
            ]
        })
    ]
}

// ------ footer ------

fn view_footer(todos: &BTreeMap<Ulid, Todo>, selected_filter: Filter) -> Node<Msg> {
    let completed_count = todos.values().filter(|todo| todo.completed).count();
    let active_count = todos.len() - completed_count;

    footer![C!["footer"],
        span![C!["todo-count"],
            strong![active_count],
            format!(" item{} left", if active_count == 1 { "" } else { "s" }),
        ],
        view_filters(selected_filter),
        IF!(completed_count > 0 =>
            button![C!["clear-completed"],
                "Clear completed",
                ev(Ev::Click, |_| Msg::ClearCompleted),
            ]
        )
    ]
}

fn view_filters(selected_filter: Filter) -> Node<Msg> {
    ul![C!["filters"],
        Filter::iter().map(|filter| {
            let (link, title) = match filter {
                Filter::All => ("#/", "All"),
                Filter::Active => ("#/active", "Active"),
                Filter::Completed => ("#/completed", "Completed"),
            };
            li![
                a![C![IF!(filter == selected_filter => "selected")],
                    attrs!{At::Href => link},
                    title,
                ],
            ]
        })
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    
    let root_element = document()
        .get_elements_by_class_name("todoapp")
        .item(0)
        .expect("element with the class `todoapp`");

    App::start(root_element, init, update, view);
}
