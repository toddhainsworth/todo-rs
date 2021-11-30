use serde_json;
use std::fs;
use std::io::Result;
use home::home_dir;

const TODO_FILENAME: &'static str = ".todos";

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: usize,
    pub text: String,
    pub completed: bool,
}

impl Item {
    pub fn new(id: usize, text: &str, completed: bool) -> Self {
        Self {
            id,
            text: String::from(text),
            completed,
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Item::new(0, "", false)
    }
}

pub struct ItemRepository {
    items: Vec<Item>,
}

impl ItemRepository {
    pub fn new() -> Result<Self> {
        let item_json = ItemRepository::load_items()?;
        let items: Vec<Item> = serde_json::from_str(&item_json)?;
        Ok(Self { items })
    }

    pub fn delete(&mut self, id: usize) {
        self.items.retain(|item| item.id != id)
    }

    pub fn publish(self) -> Result<()> {
        let path = Self::get_todo_file_path();
        let buf = serde_json::to_string(&self.items).unwrap();
        fs::write(path, buf)
    }

    pub fn toggle(&mut self, id: usize) {
        self.items
            .iter_mut()
            .find(|item| item.id == id)
            .map(|item| item.completed = !item.completed);
    }

    pub fn update_text(&mut self, id: usize, text: &str) {
        self.items
            .iter_mut()
            .find(|item| item.id == id)
            .map(|item| item.text = text.to_string());
    }

    pub fn add(&mut self, text: &str) {
        let id = self.items.iter().map(|item| item.id).max().unwrap_or(0) + 1;
        self.items.push(Item::new(id, text, false))
    }

    pub fn items(&mut self) -> &mut Vec<Item> {
        self.items.as_mut()
    }

    fn load_items() -> Result<String> {
        fs::read_to_string(Self::get_todo_file_path())
    }

    fn get_todo_file_path() -> String {
        // unwrapping because if this were to fail then there's something _really_ wrong with the users setup...
        let home = home_dir().unwrap();
       format!("{}/{}", home.display(), TODO_FILENAME)
    }
}
