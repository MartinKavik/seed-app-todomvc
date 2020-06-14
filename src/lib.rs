#![allow(clippy::wildcard_imports)]
// TODO: Remove
#![allow(dead_code, unused_variables)]

use seed::{prelude::*, *};
use std::collections::BTreeMap;
use ulid::Ulid;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        todos: BTreeMap::new(),
        new_todo_title: String::new(),
        selected_todo: None,
        filter: Filter::All,
        base_url: Url::new(),
    }.add_mock_data()
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

// TODO: Remove
impl Model {
    fn add_mock_data(mut self) -> Self {
        let (id_a, id_b) = (Ulid::new(), Ulid::new());
        
        self.todos.insert(id_a, Todo {
            id: id_a,
            title: "I'm todo A".to_owned(),
            completed: false,
            element: ElRef::new()
        });

        self.todos.insert(id_b, Todo {
            id: id_b,
            title: "I'm todo B".to_owned(),
            completed: true,
            element: ElRef::new()
        });

        self.new_todo_title = "I'm a new todo title".to_owned();

        self.selected_todo = Some(SelectedTodo {
            id: id_b,
            title: "I'm better todo B".to_owned(),
        });
        self
    }
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

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        view_header(),
        IF!(not(model.todos.is_empty()) => vec![
            view_main(), 
            view_footer(),
        ]),
    ]
}

// ------ header ------

fn view_header() -> Node<Msg> {
    header![C!["header"],
        h1!["todos"],
        input![C!["new-todo"],
            attrs!{At::Placeholder => "What needs to be done?", At::AutoFocus => AtValue::None},
        ]
    ]
}

// ------ main ------

fn view_main() -> Node<Msg> {
    section![C!["main"],
        input![C!["toggle-all"], attrs!{At::Id => "toggle-all", At::Type => "checkbox"}],
        label![attrs!{At::For => "toggle-all"}, "Mark all as complete"],
        view_todo_list(),
    ]
}

fn view_todo_list() -> Node<Msg> {
    ul![C!["todo-list"],
        // These are here just to show the structure of the list items
        // List items should get the class `editing` when editing and `completed` when marked as completed
        li![C!["completed"],
            div![C!["view"],
                input![C!["toggle"], attrs!{At::Type => "checkbox", At::Checked => AtValue::None}],
                label!["Taste JavaScript"],
                button![C!["destroy"]],
            ],
            input![C!["edit"], attrs!{At::Value => "Create a TodoMVC template"}]
        ],
        li![
            div![C!["view"],
                input![C!["toggle"], attrs!{At::Type => "checkbox"}],
                label!["Buy a unicorn"],
                button![C!["destroy"]],
            ],
            input![C!["edit"], attrs!{At::Value => "Rule the web"}]
        ]
    ]
}

// ------ footer ------

fn view_footer() -> Node<Msg> {
    footer![C!["footer"],
        // This should be `0 items left` by default
        span![C!["todo-count"],
            strong!["0"],
            " item left",
        ],
        view_filters(),
        // Hidden if no completed items are left ↓
        button![C!["clear-completed"],
            "Clear completed"
        ]
    ]
}

fn view_filters() -> Node<Msg> {
    ul![C!["filters"],
        li![
            a![C!["selected"],
                attrs!{At::Href => "#/"},
                "All",
            ],
        ],
        li![
            a![
                attrs!{At::Href => "#/active"},
                "Active",
            ],
        ],
        li![
            a![
                attrs!{At::Href => "#/completed"},
                "Completed",
            ],
        ],
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
