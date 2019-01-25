use std::io::Result;
use std::path::Path;
use std::fs;
use dirs;
use serde_json;

const TODO_FILENAME: &'static str = ".todos";

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub text: String,
    pub completed: bool,
    pub priority: usize
}

impl TodoItem {
    pub fn new(text: &str, completed: bool, priority: usize) -> Self {
        TodoItem { text: String::from(text), completed, priority }
    }

    pub fn toggle_complete(&mut self) {
        self.completed = !self.completed;
    }
}

impl Default for TodoItem {
    fn default() -> Self {
        TodoItem::new("", false, 1)
    }
}

pub fn get_todo_file() -> Result<String> {
    fs::read_to_string(get_todo_file_path())
}

pub fn get_todo_file_path() -> String {
    let home = dirs::home_dir().unwrap();
    format!("{}/{}", home.display(), Path::new(TODO_FILENAME).display())
}

pub fn update_todo_file(items: &Vec<TodoItem>) -> Result<()> {
    let path = get_todo_file_path();
    let buf = serde_json::to_string(&items).unwrap();
    fs::write(path, buf)
}

