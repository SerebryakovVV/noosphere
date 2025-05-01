use std::{cell::RefCell, rc::Rc};
use dioxus::prelude::*;
use rusqlite::{self, params};


// add scrolling
// delete the tooltips for the input field

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
    
    fn query_books() {}
    
    fn query_timers() {}
    
}



fn main() {
    dioxus::launch(App);
}


#[component]
fn App() -> Element {

    let db = DB {connection: Rc::new(RefCell::new(rusqlite::Connection::open(DB_PATH).unwrap()))};
    use_context_provider(|| db.clone());
    let tasks = use_signal(|| db.query_tasks());
    use_context_provider(|| tasks);
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
    println!("this bitch rerenderes");
    // get the db context, the tasks context, add enter listener, add to the tasks array on success
    // dioxus::events::HasKeyboardData::key()
    rsx!{
        input {
            id: "input-bar",
            value: "{task}", 
            oninput: move |event| task.set(event.value()),
            onkeydown: move |event| {
                if event.code() == Code::Enter {
                    println!("true tho");

                    let task_to_add = task.read().clone();
                    let trimmed = task_to_add.trim();
                    if !trimmed.is_empty() {
                        match db.add_task(trimmed) {
                            Ok(new_task_id) => {
                                // if ok then we just add new task with id we got and trimmed
                                tasks.write().push(Task { id: new_task_id, text: trimmed.to_string() });
                                task.set(String::new());
                            },
                            Err(_) => {}
                        }
                    }


                }
                // println!("{}", event.code());
                
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
    // let db = use_context::<DB>();
    // Signal<Vec<Task>>
    // let tasks = use_context::<Vec<Task>>();
    let tasks = use_context::<Signal<Vec<Task>>>();
    rsx!(
        div {
            id: "tasks-page", 
            for (index, t) in tasks.read().iter().enumerate() {
                TaskComponent{
                    id: t.id,
                    text: t.text.clone(),
                    index: index
                }
            }
            InputBar { }
        }
    )
}


#[component]
fn TaskComponent(id: i64, text: String, index: usize) -> Element {
    // an arrow showing if it has child elements
    // a number
    // text
    // delete 
    // complete
    // tags

    // use context here and get the function for deleting a row
    let mut tasks = use_context::<Signal<Vec<Task>>>();
    let db = use_context::<DB>();

    rsx!(
        div { 
            id: "task-component",
            "{index + 1}) {text}"
            div {
                id: "task-controls-container", 
                // div { id:"task-complete" }
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