// CREATE TABLE books (id INTEGER PRIMARY KEY AUTOINCREMENT, text TEXT, current INTEGER NOT NULL, length INTEGER NOT NULL);
// CREATE TABLE tasks (id INTEGER PRIMARY KEY AUTOINCREMENT, text TEXT);




#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{self, style::Widgets, Color32, FontFamily, FontId, Margin, TextStyle, Widget};
use egui::{Key, ScrollArea};
use core::task;
use std::time::{Duration, Instant};
use rusqlite::{self, params, Connection, Result as ResultSql};



const DB_PATH: &str = "tasks.db3";



#[derive(Debug)]
struct Task {
    id: i64,
    text: String
}



struct TasksApp {
    new_task: String,
    tasks: Vec<Task>,
    connection: rusqlite::Connection
}

impl Default for TasksApp {
    fn default() -> Self {
        Self {
            new_task: String::new(),
            tasks: Vec::<Task>::new(),
            connection: rusqlite::Connection::open(DB_PATH).unwrap_or_else(|e| {
                println!("{}", e);
                panic!();
            })
        }
    }
}

// a bottom panel with input and enter listener
// a list with:
//      text
//      editable
//      deletion
//      index
//      putting into archive, maybe

fn main() -> eframe::Result {
    let mut app = TasksApp {
        new_task: String::new(),
        tasks: Vec::<Task>::new(),
        connection: rusqlite::Connection::open(DB_PATH).unwrap_or_else(|e| {
            println!("{}", e);
            panic!();
        })
    };
    {
        let mut statement = app.connection.prepare("select id, text from tasks").unwrap_or_else(|e| {
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
        for row in rows {
            app.tasks.push(row.unwrap());
        };
    }
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tasks",
        options,
        Box::new(|_cc| Ok(Box::new(app)))
    )
}




impl eframe::App for TasksApp {


    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::bottom("input").show(ctx, |ui| {
                let response = ui.add_sized(
                    [ui.available_width(), 26.0], 
                    egui::TextEdit::singleline(&mut self.new_task)
                        .font(TextStyle::Heading)
                        .hint_text("Enter a task...")
                        .text_color(Color32::BLACK)                        
                );
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if self.new_task.trim().len() != 0 {
                        
                        
                        // add task to the db here
                        // get the id

                        let id: i64 = self.connection.query_row(
                            "insert into tasks (text) values (?1) returning id", 
                            params![self.new_task], 
                            |row| row.get(0)
                        ).unwrap_or_else(|e| {
                            println!("{}", e);
                            panic!();
                        });
                        


                        self.tasks.push(
                            Task { id: id, text: self.new_task.clone() }
                        );


                        self.new_task.clear();                        
                    }
                }
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut i_to_remove: Option<usize> = None;


            for (i, t) in self.tasks.iter().enumerate() {
                ui.horizontal(|ui| {

                    // if ui.button("x").clicked() {
                        // i_to_remove = Some(i);
                    // }

                    let delete_button = egui::Button::new(
                        egui::RichText::new("X")
                            .size(18.0)
                            .color(Color32::WHITE) 
                            .strong()
                    ).fill(Color32::DARK_RED)     
                     .stroke(egui::Stroke::new(1.0, Color32::BLACK)) 
                     .corner_radius(3.0)                 
                     .min_size(egui::vec2(24.0, 24.0));
                    
                    if ui.add(delete_button).clicked() {
                        i_to_remove = Some(i);
                    }

                    ui.label(
                        egui::RichText::new(format!("{}) {}", i+1, t.text))
                            .font(FontId::new(20.0, FontFamily::Proportional))
                            .color(Color32::BLACK)
                    );
                });
            }
            if let Some(i) = i_to_remove {
                // with the "i" i get a task from the array and get its id and delete
                self.connection.execute("delete from tasks where id = ?1", params![self.tasks[i].id]).unwrap_or_else(|e| {
                    println!("{}", e);
                    panic!();
                });
                self.tasks.remove(i);
            }
            
        });
    }
}


// impl TasksApp {
//     fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         let mut style = egui::Style::default();
//         style.visuals.override_text_color = Some(egui::Color32::DARK_RED);
        
//         cc.egui_ctx.set_style(style);

//         Self {
//             new_task: String::new(),
//             tasks: Vec::new(),
//         }
//     }
// }


// impl MyEguiApp {
//     fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
//         // Restore app state using cc.storage (requires the "persistence" feature).
//         // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
//         // for e.g. egui::PaintCallback.
//         Self::default()
//     }
// }


 // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     let task_inp = ui.text_edit_singleline(&mut self.inp_str)
            //         .labelled_by(name_label.id);
            //     if ui.button("add").clicked() && self.inp_str.trim().len() != 0 {
            //         self.tasks.push(self.inp_str.clone());
            //     }
            // });
            
            // ui.label(self.numb.to_string());

            // for s in &self.tasks {
            //     ui.label(s);
            // }
            
            // ui.label(self.time.elapsed().as_secs().to_string());
            // ScrollArea::vertical()
            //     .auto_shrink(false)
            //     .stick_to_bottom(true)
            //     .show(ui, |ui| {
            //         ui.label(&self.text);
            //     });

            // if ctx.input(|i| i.key_pressed(Key::A)) {
            //     self.text.push_str("\nPressed");
            // }
            // if ctx.input(|i| i.key_down(Key::A)) {
            //     self.text.push_str("\nHeld");
            //     ui.ctx().request_repaint(); // make sure we note the holding.
            // }
            // if ctx.input(|i| i.key_released(Key::A)) {
            //     self.text.push_str("\nReleased");
            // }


    // ui.text_edit_singleline(&mut self.new_task);
                // if ui.button("Add").clicked() {
                //     self.tasks.push(self.new_task.clone());
                //     self.new_task.clear();
                // }

                // let input_text_style = TextStyle {};


// db.execute("DROP TABLE users", ())?;

    // let query_result = match db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY,name  TEXT NOT NULL)", ()) {
    //     Err(e) => {println!("{}", e); panic!("first");},
    //     Ok(r) => r 
    // };

    // let query_result = match db.execute("insert into users (name) values ('valentin')", ()) {
    //     Err(e) => {println!("{}", e); panic!("first");},
    //     Ok(r) => r 
    // };

    // let mut statement = db.prepare("select name from users")?;
    // let query_result = statement.query_map([], |el| {
    //     Ok(User{name:el.get(0)?})
    // })?;

  

    // for u in query_result {
    //     println!("{:?}", u);
        
    // }





// mod expressions;

// #[derive(Debug)]
// struct User<'a> {
//     // name: String
//     name: & 'a str
// }


// const COMMAND_CAPACITY: usize = 500;
// const DB_PATH: &str = "tasks.db3";


// struct Task {
//     id: i32,
//     name: String   // learn lifetimes, get rid of the allocations
// }


// fn add_task(arg: &str, con: &rsql::Connection) {
//     println!("add task {}", arg);

//     if false {
        
//     }

// }


// fn delete_task(arg: &str, con: &rsql::Connection) {
//     println!("delete_task {}", arg);

//     if false {
        
//     }

// }


// fn list_tasks(con: &rsql::Connection) {
//     let mut statement = match con.prepare("SELECT id, name FROM tasks") {
//         Ok(s) => s,
//         Err(e) => {
//             println!("Error preparing ls query! {}", e);
//             return;
//         }
//     };
//     match statement.query_map([], |row| Ok(
//         Task {
//             id: row.get(0)?,
//             name: row.get(1)?
//         }
//     )) {
//         Ok(rows) => {
//             for (index, row) in rows.enumerate() {
//                 match row {
//                     Ok(r) => {
//                         println!("{}) {}", index + 1, r.name);
//                     },
//                     Err(e) => {
//                         println!("Error in rows transfer in ls, {}", e);
//                         return;
//                     }
//                 }
//             }
//         },
//         Err(e) => {
//             println!("Error mapping the rows in ls, {}", e);
//             return;
//         }
//     };
// }


// fn main() -> rsql::Result<()> {
//     let db = rsql::Connection::open(DB_PATH).expect("error connecting to db");
//     let mut command: String = String::with_capacity(COMMAND_CAPACITY);
//     let input = std::io::stdin();
//     loop {
//         if let Err(e) = input.read_line(&mut command) {
//             println!("Error reading the input! {}", e);
//             continue;
//         };
//         let command_parts: Vec<&str> = command
//                                               .trim()
//                                               .split_ascii_whitespace()
//                                               .collect();                    
//         match command_parts.get(0) {
//             Some(&c) => {
//                 match c {
//                     "delete" => {
//                         match command_parts.get(1) {
//                             Some(&a) => delete_task(a, &db),
//                             None => println!("No argument provided!")
//                         }
//                     },
//                     "add" => {
//                         match command_parts.get(1) {
//                             Some(&a) => add_task(a, &db),
//                             None => println!("No argument provided!") // this should actually just call the function and 
//                         }                                             // then read the task inside to that function
//                     },
//                     "ls" => list_tasks(&db),
//                     "q" => {return Ok(());},
//                     _ => println!("Unknown command")
//                 }
//             },
//             None => println!("No command provided!")
//         }
//         command.clear();
//         // print!("> ");
//     }
// }




// db.execute("DROP TABLE users", ())?;

    // let query_result = match db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY,name  TEXT NOT NULL)", ()) {
    //     Err(e) => {println!("{}", e); panic!("first");},
    //     Ok(r) => r 
    // };

    // let query_result = match db.execute("insert into users (name) values ('valentin')", ()) {
    //     Err(e) => {println!("{}", e); panic!("first");},
    //     Ok(r) => r 
    // };

    // let mut statement = db.prepare("select name from users")?;
    // let query_result = statement.query_map([], |el| {
    //     Ok(User{name:el.get(0)?})
    // })?;

  

    // for u in query_result {
    //     println!("{:?}", u);
        
    // }
