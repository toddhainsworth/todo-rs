extern crate serde;
extern crate serde_json;
extern crate colored;
extern crate touch;
extern crate dirs;

#[macro_use]
extern crate serde_derive;

use std::env;
use std::process;
use colored::*;
use touch::exists;

mod todo_item;

use todo_item::TodoItem;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Guard against having no todo file
    if !exists(&todo_item::get_todo_file_path()) && args.len() > 1 && args[1] != "init" {
        eprintln!("Could not load todo file, run with `{} init` first", args[0]);
        process::exit(1);
    }

    // Setup intiial requirements
    if args.len() == 2 && args[1] == "init" {
        if exists(&todo_item::get_todo_file_path()) {
            process::exit(0);
        }

        match todo_item::update_todo_file(&Vec::new()) {
            Ok(_) => process::exit(0),
            Err(e) => {
                eprintln!("Could not create todo file: {}", e);
                process::exit(1);
            }
        }
    }

    let f = match todo_item::get_todo_file() {
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

    // Sort items by priority 1 = highest, Infinity = lowest
    items.sort_by(|a, b| a.priority.cmp(&b.priority));

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
                    Some(item) => item.toggle_complete(),
                    None => ()
                };
            },
            Err(e) => {
                eprintln!("Could not mark item as complete: {}", e);
                process::exit(1);
            }
        };
    // Edit existing item
    } else if args.len() == 4 && args[1] == "-e" {
        let item_id = &args[2];
        match item_id.parse::<usize>() {
            Ok(id) => {
                match items.get_mut(id) {
                    Some(item) => item.text = args[3].clone(),
                    None => ()
                };
            },
            Err(e) => {
                eprintln!("Could not edit item: {}", e);
                process::exit(1);
            }
        };
    // Change priority of item
    } else if args.len() == 4 && args[1] == "-p" {
        let item_id = &args[2];

        match item_id.parse::<usize>() {
            Ok(id) => {
                if let Some(item) = items.get_mut(id) {
                    match args[3].parse::<usize>() {
                        Ok(p) => item.priority = p,
                        Err(_) => ()
                    };
                }
            },
            Err(e) => {
                eprintln!("Could not edit item: {}", e);
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

    match todo_item::update_todo_file(&items) {
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

fn print_usage(args: &Vec<String>) {
    println!("USAGE: {0} ...

EXAMPLES:
Show current items
- {0}
Add a new item
- {0} \"Do something cool\"
- {0} \"Submit report for XYZ\" 0 # top priority
- {0} \"Get milk on the way home\" 4 # lower priority
Mark an existing item as complete
- {0} -c <item-id>
Delete an existing item from the list
- {0} -d <item-id>
Edit an existing item
- {0} -e <item-id> \"Do some thing REALLY cool!\"
Change the priority of an item
- {0} -p <item-id> 42
", args[0]);
}
