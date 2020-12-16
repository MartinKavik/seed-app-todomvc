#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::mem;

use seed_routing::*;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use ulid::Ulid;

const ENTER_KEY: &str = "Enter";
const ESCAPE_KEY: &str = "Escape";

const STORAGE_KEY: &str = "todos-seed";

add_router!();
// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    router()
        .init(url)
        .set_handler(orders, move |subs::UrlRequested(requested_url, _)| {
            router().confirm_navigation(requested_url)
        });

    Model {
        todos: LocalStorage::get(STORAGE_KEY).unwrap_or_default(),
        new_todo_title: String::new(),
        selected_todo: None,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    todos: BTreeMap<Ulid, Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>,
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

#[derive(Copy, Debug, Clone, Eq, PartialEq, ParseUrl, Root, EnumIter)]
enum Route {
    Active,
    Completed,
    #[default_route]
    NotFound,
    #[as_path = ""]
    All,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
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
        Msg::NewTodoTitleChanged(title) => {
            model.new_todo_title = title;
        }
        // ------ Basic Todo operations ------
        Msg::CreateTodo => {
            let title = model.new_todo_title.trim();
            if not(title.is_empty()) {
                let id = Ulid::new();
                model.todos.insert(
                    id,
                    Todo {
                        id,
                        title: title.to_owned(),
                        completed: false,
                    },
                );
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

                    input_element.focus().expect("focus input_element");

                    input_element
                        .set_selection_range(title_length, title_length)
                        .expect("move cursor to the end of input_element");
                });
            }
        }
        Msg::SelectTodo(None) => {
            model.selected_todo = None;
        }
        Msg::SelectedTodoTitleChanged(title) => {
            if let Some(selected_todo) = &mut model.selected_todo {
                selected_todo.title = title;
            }
        }
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
    if router().current_route() == router().default_route() {
        vec![div![
            "page not found ",
            br![],
            C!["view"],
            a![
                "Click me to find your items! ",
                attrs! {At::Href => Route::All.to_url()},
            ]
        ]]
    } else {
        nodes![
            view_header(&model.new_todo_title),
            IF!(not(model.todos.is_empty()) => vec![
                view_main(&model.todos, model.selected_todo.as_ref(), router().current_route()),
                view_footer(&model.todos),
            ])
        ]
    }
}

// ------ header ------

fn view_header(new_todo_title: &str) -> Node<Msg> {
    header![
        C!["header"],
        h1!["todos"],
        input![
            C!["new-todo"],
            attrs! {
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

fn view_main(
    todos: &BTreeMap<Ulid, Todo>,
    selected_todo: Option<&SelectedTodo>,
    route: Route,
) -> Node<Msg> {
    section![
        C!["main"],
        view_toggle_all(todos),
        view_todo_list(todos, selected_todo, route),
    ]
}

fn view_toggle_all(todos: &BTreeMap<Ulid, Todo>) -> Vec<Node<Msg>> {
    let all_completed = todos.values().all(|todo| todo.completed);
    vec![
        input![
            C!["toggle-all"],
            attrs! {
                At::Id => "toggle-all", At::Type => "checkbox", At::Checked => all_completed.as_at_value()
            },
            ev(Ev::Change, |_| Msg::CheckOrUncheckAll),
        ],
        label![attrs! {At::For => "toggle-all"}, "Mark all as complete"],
    ]
}

// TODO: Remove once rustfmt is updated.
#[rustfmt::skip]
fn view_todo_list(
    todos: &BTreeMap<Ulid, Todo>,
    selected_todo: Option<&SelectedTodo>,
    route: Route,
) -> Node<Msg> {
    let todos = todos.values().filter(|todo| match route {
        Route::All => true,
        Route::Active => not(todo.completed),
        Route::Completed => todo.completed,
        Route::NotFound => true
    });
    ul![C!["todo-list"],
        todos.map(|todo| {
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

fn view_footer(todos: &BTreeMap<Ulid, Todo>) -> Node<Msg> {
    let completed_count = todos.values().filter(|todo| todo.completed).count();
    let active_count = todos.len() - completed_count;

    footer![
        C!["footer"],
        span![
            C!["todo-count"],
            strong![active_count],
            format!(" item{} left", if active_count == 1 { "" } else { "s" }),
        ],
        view_filters(),
        IF!(completed_count > 0 =>
            button![C!["clear-completed"],
                "Clear completed",
                ev(Ev::Click, |_| Msg::ClearCompleted),
            ]
        )
    ]
}

fn view_filters() -> Node<Msg> {
    ul![
        C!["filters"],
        Route::iter()
            .filter(|route| route != &Route::NotFound)
            .map(|filter| {
                let title = match filter {
                    Route::All => "All",
                    Route::Active => "Active",
                    Route::Completed => "Completed",
                    Route::NotFound => "",
                };
                li![a![
                    C![IF!(filter == router().current_route() => "selected")],
                    attrs! {At::Href => filter.to_url()},
                    title,
                ],]
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
