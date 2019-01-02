extern crate serde;
extern crate serde_json;
extern crate dirs;
extern crate colored;

#[macro_use]
extern crate serde_derive;

use std::path::Path;
use std::fs;
use std::env;
use std::io::Result;
use std::process;
use colored::*;

const TODO_FILENAME: &'static str = ".todos";

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    text: String,
    completed: bool
}

impl TodoItem {
    fn new(text: &str, completed: bool) -> Self {
        TodoItem { text: String::from(text), completed }
    }
}

impl Default for TodoItem {
    fn default() -> Self {
        TodoItem::new("", false)
    }
}

fn main() {
    let f = match get_todo_file() {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Could not read todo file: {}", e);
            process::exit(1);
        }
    };

    let mut items: Vec<TodoItem> = serde_json::from_str(&f).unwrap();
    let args: Vec<String> = env::args().collect();

    // Delete items
    if args.len() >= 3 && args[1] == "-d" {
        let item_id = &args[2];

        match item_id.parse::<usize>() {
            Ok(id) => items.remove(id),
            Err(e) => {
                eprintln!("Could not mark item as complete: {}", e);
                process::exit(1);
            }
        };
    }

    // Toggle completion of items
    if args.len() == 3 && args[1] == "-c" {
        let item_id = &args[2];
        match item_id.parse::<usize>() {
            Ok(id) => {
                match items.get_mut(id) {
                    Some(item) => item.completed = !item.completed,
                    None => ()
                };
            },
            Err(e) => {
                eprintln!("Could not mark item as complete: {}", e);
                process::exit(1);
            }
        };
    }

    // Clear all or add a new one
    if args.len() == 2 {
        let text = &args[1];
        items.push(TodoItem::new(text, false))
    }

    match update_todo_file(&items) {
        Err(e) => {
            eprintln!("Failed to update todo file: {}", e);
            process::exit(1);
        },
        _ => ()
    }

    for (i, item) in items.into_iter().enumerate() {
        let text = if item.completed {
            item.text.green()
        } else {
            item.text.yellow()
        };

        println!("{} - {}", i, text);
    }
}

fn get_todo_file() -> Result<String> {
    fs::read_to_string(get_todo_file_path())
}

fn get_todo_file_path() -> String {
    let home = dirs::home_dir().unwrap();
    format!("{}/{}", home.display(), Path::new(TODO_FILENAME).display())
}

fn update_todo_file(items: &Vec<TodoItem>) -> Result<()> {
    let path = get_todo_file_path();
    let buf = serde_json::to_string(&items).unwrap();
    fs::write(path, buf)
}
