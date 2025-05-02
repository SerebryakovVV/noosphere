use std::{cell::RefCell, rc::Rc};
use dioxus::{html::div, prelude::*};
use rusqlite::{self, params};

use dioxus::desktop::WindowBuilder;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const DB_PATH: &str = "./noosphere_db.db3";


enum Page {
    Tasks,
    Books,
    Timers
}


#[derive(Clone)]
struct Task {
    id: i64,
    text: String
}

struct Book {
    id: i64,
    name: String,
    current: i32,
    length: i32
}

#[derive(Clone)]
struct DB {
    connection: Rc<RefCell<rusqlite::Connection>>
}


impl DB {
    fn query_tasks(&self) -> Vec<Task> {
        let connection_borrow = self.connection.borrow_mut();
        let mut statement = connection_borrow.prepare("select id, text from tasks").unwrap_or_else(|e| {
            println!("{}", e);
            panic!();
        });
        let rows = statement.query_map([], |row| {
            Ok(
                Task {
                    id: row.get(0)?,
                    text: row.get(1)?
                }
            )
        }).unwrap_or_else(|e| {
            println!("{}", e);
            panic!();
        });
        let mut res_arr = Vec::<Task>::new();
        for row in rows {
            res_arr.push(row.unwrap());
        };
        res_arr
    }
    
    fn delete_task(&self, id: i64) -> Result<usize, rusqlite::Error> {
        self.connection.borrow().execute("delete from tasks where id = ?1", params![id])
    }
    
    fn add_task(&self, text: &str) -> Result<i64, rusqlite::Error> {
        self.connection.borrow().query_row(
            "insert into tasks (text) values (?1) returning id", 
            params![text], 
            |row| row.get(0)
        )
    }
    
    fn query_books(&self) -> Vec<Book> {
        let connection_borrow = self.connection.borrow_mut();
        let mut statement = connection_borrow.prepare("select id, text, current, length from books").unwrap_or_else(|e| {
            println!("{}", e);
            panic!();
        });
        let rows = statement.query_map([], |row| {
            Ok(
                Book {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    current: row.get(2)?,
                    length: row.get(3)?
                }
            )
        }).unwrap_or_else(|e| {
            println!("{}", e);
            panic!();
        });
        let mut res_arr = Vec::<Book>::new();
        for row in rows {
            res_arr.push(row.unwrap());
        };
        res_arr
    }
    
    fn query_timers() {}
}


fn main() {
    dioxus
        ::LaunchBuilder
        ::desktop()
        .with_cfg(
            dioxus
            ::desktop
            ::Config
            ::default()
            .with_window(
                WindowBuilder
                ::new()
                // .with_always_on_top(false)
                .with_always_on_top(true)
            )
        ).launch(App);
}


#[component]
fn App() -> Element {
    let db = DB {connection: Rc::new(RefCell::new(rusqlite::Connection::open(DB_PATH).unwrap()))};
    use_context_provider(|| db.clone());
    let tasks = use_signal(|| db.query_tasks());
    use_context_provider(|| tasks);

    let books = use_signal(|| db.query_books());
    use_context_provider(|| books);

    let mut page = use_signal(|| Page::Tasks);
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div { 
            id: "root-container",
            div {
                id: "tabs-sidebar",
                div { class:"sidebar-button", onclick: move |_| page.set(Page::Tasks), "Tasks" }
                div { class:"sidebar-button", onclick: move |_| page.set(Page::Books), "Books" }
                div { class:"sidebar-button", onclick: move |_| page.set(Page::Timers), "Timers" }
            }
            div {
                id: "content-container",
                match *page.read() {
                    Page::Tasks => rsx!(TasksPage { }),
                    Page::Books => rsx!(BooksPage {  }),
                    Page::Timers => rsx!(TimersPage {  })
                }
            }
        }
    }
}


#[component]
fn InputBar() -> Element {
    let mut task = use_signal(|| "".to_string());
    let mut tasks = use_context::<Signal<Vec<Task>>>();
    let db = use_context::<DB>();
    rsx!{
        input {
            id: "input-bar",
            value: "{task}", 
            autocomplete: "off",
            oninput: move |event| task.set(event.value()),
            onkeydown: move |event| {
                if event.code() == Code::Enter {
                    let task_to_add = task.read().clone();
                    let trimmed = task_to_add.trim();
                    if !trimmed.is_empty() {
                        match db.add_task(trimmed) {
                            Ok(new_task_id) => {
                                tasks.write().push(Task { id: new_task_id, text: trimmed.to_string() });
                                task.set(String::new());
                            },
                            Err(_) => {}
                        }
                    }
                }
            },
            placeholder: "Input the task..." 
        }
    }
}


#[component]
fn TabsBar() -> Element {
    rsx!(
        div {"tabs here"}
    )
}


#[component]
fn TasksPage() -> Element {
    let tasks = use_context::<Signal<Vec<Task>>>();
    rsx!(
        div {
            id: "tasks-page", 
            div { 
                id: "tasks-page-list",
                for (index, t) in tasks.read().iter().enumerate() {
                    TaskComponent{
                        id: t.id,
                        text: t.text.clone(),
                        index: index
                    }
                }                
            }
            InputBar { }
        }
    )
}


#[component]
fn TaskComponent(id: i64, text: String, index: usize) -> Element {
    let mut tasks = use_context::<Signal<Vec<Task>>>();
    let db = use_context::<DB>();
    rsx!(
        div { 
            id: "task-component",
            "{index + 1}) {text}"
            div {
                id: "task-controls-container", 
                div { 
                    id:"task-delete", 
                    onclick: move |_| {
                        match db.delete_task(id) {
                            Ok(_) => {
                                tasks.write().remove(index);
                            },
                            Err(_) => {}
                        };
                    } 
                }
            }
        }
    )
}


#[component]
fn BooksPage() -> Element {
    let books = use_context::<Signal<Vec<Book>>>();
    rsx!(
        div {
            id: "books-page",
            for (index, b) in books.read().iter().enumerate() {
                BookComponent{
                    id: b.id,
                    name: b.name.clone(),
                    current: b.current,
                    length: b.length,
                    index: index
                }
            }
        }
    )
}


#[component]
fn BookComponent(id: i64, name: String, current: i32, length: i32, index: usize) -> Element {
    let mut current_page_state_string = use_signal(|| String::from(""));
    let mut done_div_width = use_signal(|| 0);
    rsx!(
        div { 
            id: "book-component",
            div {
                id: "book-name",
                "{index + 1}) {name}"
            }
            div {
                id: "book-progress-container",
                input {  
                    id: "page-count-done",
                    value: "{current_page_state_string}",
                    oninput: move |event| {
                        let new_current_page_string = event.value()
                                                           .chars()
                                                           .filter(|x| x.is_ascii_digit())
                                                           .take(5)
                                                           .collect::<String>()
                                                           .trim_start_matches('0').to_string();
                        if new_current_page_string.len() == 0 {
                            done_div_width.set(0);
                            current_page_state_string.set(String::from(""));
                            return;
                        }
                        let new_current_page_number = new_current_page_string.parse::<i32>().unwrap();
                        if new_current_page_number >= length {
                            done_div_width.set(100);
                            current_page_state_string.set(length.to_string());
                            return;
                        } 
                        done_div_width.set(
                            ((new_current_page_number as f32 / length  as f32) * 100.0) as i32
                        );
                        current_page_state_string.set(new_current_page_string);
                    },
                    "{current}"
                }
                div {
                    id: "progress-bar",
                    div {
                        id: "progress-bar-done",
                        style: "width: {done_div_width}%"
                    }
                }
                div {  
                    id: "page-count-left",
                    "{length}"
                }
            }
        }
    )
}


#[component]
fn TimersPage() -> Element {
    rsx!(
        div {"timers here"}
    )
}