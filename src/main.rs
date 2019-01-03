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
    completed: bool,
    priority: usize
}

impl TodoItem {
    fn new(text: &str, completed: bool, priority: usize) -> Self {
        TodoItem { text: String::from(text), completed, priority }
    }
}

impl Default for TodoItem {
    fn default() -> Self {
        TodoItem::new("", false, 1)
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

    let mut items: Vec<TodoItem> = match serde_json::from_str(&f) {
        Ok(items) => items,
        Err(_) => Vec::new()
    };
    let args: Vec<String> = env::args().collect();

    // Delete items
    if args.len() >= 3 && args[1] == "-d" {
        let item_id = &args[2];
        let item_id = match item_id.parse::<usize>() {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Could not mark item as complete: {}", e);
                process::exit(1);
            }
        };

        if item_id >= items.len() {
            eprintln!("Could not find item with id: {}", item_id);
            process::exit(1);
        }

        items.remove(item_id);
    // Toggle completion of items
    } else if args.len() == 3 && args[1] == "-c" {
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
    // Print usage
    } else if args.len() == 3 && args[1] == "-h" {
        print_usage(&args);
        process::exit(0);
    // Add a new item
    } else if args.len() >= 2 {
        let text = &args[1];
        // 3 args means we have a priority
        if args.len() == 3 {
            let priority = &args[2];
            match priority.parse::<usize>() {
                Ok(priority) => items.push(TodoItem::new(text, false, priority)),
                Err(e) => {
                    eprintln!("Priority must be a number (1, 2 or 3): {}", e);
                    process::exit(1);
                }
            };
        } else {
            items.push(TodoItem::new(text, false, 1));
        }
    }

    match update_todo_file(&items) {
        Err(e) => {
            eprintln!("Failed to update todo file: {}", e);
            process::exit(1);
        },
        _ => ()
    }

    // Sort items by priority 1 = highest, Infinity = lowest
    items.sort_by(|a, b| a.priority.cmp(&b.priority));

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

fn print_usage(args: &Vec<String>) {
    println!("USAGE: {0} ...

EXAMPLES:
Show current items
- {0}
Add a new item
- {0} \"Do something cool\"
- {0} \"Submit report for XYZ\" 1 # top priority
- {0} \"Get milk on the way home\" 4 # lower priority
Mark an existing item as complete
- {0} -c <item-id> # get this by running '{0}' on it's own
Delete an existing item from the list
- {0} -d <item-id> # get this by running '{0}' on it's own
    ", args[0]);
}
