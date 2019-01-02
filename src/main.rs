extern crate serde;
extern crate serde_json;
extern crate dirs;
extern crate colored;

#[macro_use]
extern crate serde_derive;

use std::path::Path;
use std::fs;
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

    let items: Vec<TodoItem> = serde_json::from_str(&f).unwrap();

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
    let home = dirs::home_dir().unwrap();
    let path = format!(
        "{}/{}", home.display(), Path::new(TODO_FILENAME).display()
    );
    fs::read_to_string(path)
}
