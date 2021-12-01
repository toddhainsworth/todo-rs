extern crate clap;
extern crate colored;
extern crate dirs;
extern crate home;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use colored::*;
use std::process;

use clap::{App, Arg};

mod item;
use item::ItemRepository;

const TODO_FILENAME: &'static str = ".todos";

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
        .arg(Arg::with_name("TEXT").help("The todo item text").index(1))
        .arg(
            Arg::with_name("complete")
                .short("c")
                .long("complete")
                .help("toggle completion of an entry")
                .takes_value(true)
                .value_delimiter(" "),
        )
        .get_matches();

    let mut repository = match ItemRepository::new(Some(TODO_FILENAME)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to load items: {}", e);
            process::exit(1);
        }
    };

    // Delete items
    if let Some(item_id) = matches.value_of("delete") {
        match item_id.parse::<usize>() {
            Ok(id) => repository.delete(id),
            Err(e) => {
                eprintln!("Could not mark item as complete: {}", e);
                process::exit(1);
            }
        };
    }

    // Toggle completion of items
    if let Some(item_id) = matches.value_of("complete") {
        match item_id.parse::<usize>() {
            Ok(id) => repository.toggle(id),
            Err(e) => {
                eprintln!("Could not mark item as complete: {}", e);
                process::exit(1);
            }
        };
    }

    if let Some(text) = matches.value_of("TEXT") {
        if let Some(item_id) = matches.value_of("edit") {
            match item_id.parse::<usize>() {
                Ok(id) => repository.update_text(id, text),
                Err(e) => {
                    eprintln!("Could not edit item: {}", e);
                    process::exit(1);
                }
            }
        } else {
            repository.add(text);
        }
    }

    for item in repository.items() {
        let text = if item.completed {
            item.text.green()
        } else {
            item.text.yellow()
        };

        println!("{} - {}", item.id, text);
    }

    if let Err(e) = repository.publish() {
        eprintln!("Failed to publish todo file: {}", e);
    }
}
