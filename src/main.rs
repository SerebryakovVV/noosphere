use std::{cell::RefCell, rc::Rc};
use dioxus::prelude::*;
use rusqlite;


const MAIN_CSS: Asset = asset!("/assets/main.css");
const DB_PATH: &str = "./noosphere_db.db3";


enum Page {
    Tasks,
    Books,
    Timers
}


struct Task {
    id: i64,
    text: String
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
    fn query_books() {}
    fn query_timers() {}
}



fn main() {
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    let db = use_hook(|| {
        DB {
            connection: Rc::new(RefCell::new(rusqlite::Connection::open(DB_PATH).unwrap()))
        }
    });
    let mut page = use_signal(|| Page::Tasks);
    let mut tasks = use_signal(|| db.query_tasks());

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
                    Page::Tasks => rsx!(TasksPage { tasks: tasks }),
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
    rsx!{
        input {
            id: "input-bar",
            value: "{task}", 
            oninput: move |event| task.set(event.value()),
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
fn TasksPage(tasks: Signal<Vec<Task>>) -> Element {
    rsx!(
        div {
            id: "tasks-page", 
            for t in tasks.read().iter() {
                TaskComponent{
                    id: t.id,
                    text: t.text.clone()
                }
            }
            InputBar { }
        }
    )
}


#[component]
fn TaskComponent(id: i64, text: String) -> Element {
    // an arrow showing if it has child elements
    // a number
    // text
    // delete 
    // complete
    // tags
    rsx!(
        div { 
            id: "task-component",
            "{id}) {text}"
            div {
                id: "task-controls-container", 
                // "hello"
                div { id:"task-complete" }
                div { id:"task-delete" }
            }
        }
    )
}



#[component]
fn BooksPage() -> Element {
    rsx!(
        div {"books here"}
    )
}


#[component]
fn TimersPage() -> Element {
    rsx!(
        div {"timers here"}
    )
}