extern crate clap;
extern crate colored;
extern crate dirs;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use colored::*;
use std::process;

use clap::{App, Arg};

mod todo_item;

use todo_item::TodoItem;

fn main() {
    let matches = App::new("Todo")
        .version("0.1.0") // TODO: Get this from the crate somehow
        .author("Todd Hainsworth <hainsworth.todd@gmail.com>")
        .about("Todo App in Rust")
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .help("delete an entry")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("edit")
                .short("e")
                .long("edit")
                .help("edit an entry")
                .value_delimiter("-"),
        )
        .arg(
            Arg::with_name("TEXT")
                .help("The todo item text")
                .index(1)
        )
        .arg(
            Arg::with_name("priority")
                .short("p")
                .long("priority")
                .help("change the priority of an entry")
                .value_delimiter(" "),
        )
        .arg(
            Arg::with_name("complete")
                .short("c")
                .long("complete")
                .help("toggle completion of an entry")
                .takes_value(true)
                .value_delimiter(" "),
        )
        .get_matches();

    let f = match todo_item::get_todo_file() {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Could not read todo file: {}", e);
            process::exit(1);
        }
    };
    let mut items: Vec<TodoItem> = match serde_json::from_str(&f) {
        Ok(items) => items,
        Err(_) => Vec::new(),
    };

    // Sort items by priority 1 = highest, Infinity = lowest
    items.sort_by(|a, b| a.priority.cmp(&b.priority));

    // Delete items
    if let Some(item_id) = matches.value_of("delete") {
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
    }

    // Toggle completion of items
    if let Some(item_id) = matches.value_of("complete") {
        match item_id.parse::<usize>() {
            Ok(id) => {
                match items.get_mut(id) {
                    Some(item) => item.toggle_complete(),
                    None => eprintln!("Could not mark item {} as complete, it doesn't exist", id)
                };
            }
            Err(e) => {
                eprintln!("Could not mark item as complete: {}", e);
                process::exit(1);
            }
        };
    }

    // Edit existing item
    if let Some(item_id) = matches.value_of("edit") {
        match item_id.parse::<usize>() {
            Ok(id) => {
                match items.get_mut(id) {
                    Some(item) => {
                        item.text = matches.value_of("TEXT").unwrap_or("EMPTY").to_string()
                    }
                    None => (),
                };
            }
            Err(e) => {
                eprintln!("Could not edit item: {}", e);
                process::exit(1);
            }
        };
    } 

    // Change priority of item
    if let Some(item_id) = matches.value_of("priority") {
        match item_id.parse::<usize>() {
            Ok(id) => {
                // FIXME: Yuck
                if let Some(item) = items.get_mut(id) {
                    if let Some(priority) = matches.value_of("TEXT") {
                        if let Ok(priority) = priority.parse::<usize>() {
                            item.priority = priority
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Could not edit item: {}", e);
                process::exit(1);
            }
        };
    }

    if let Some(text) = matches.value_of("TEXT") {
        items.push(TodoItem::new(text, false, 1));
    }

    if let Err(e) = todo_item::update_todo_file(&items) {
        eprintln!("Failed to update todo file: {}", e);
        process::exit(1);
    }

    for (i, item) in items.into_iter().enumerate() {
        let text = if item.completed {
            item.text.green()
        } else {
            item.text.yellow()
        };

        println!("{} - {}", i, text);
    }

    // this probably ins't necesarry...but it feels wrong to _assume_
    process::exit(0);
}