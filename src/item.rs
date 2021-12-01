use home::home_dir;
use serde_json;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

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
    file_path: Option<PathBuf>,
}

impl ItemRepository {
    pub fn new(path: Option<&str>) -> Result<Self> {
        let mut repo = Self {
            items: Vec::new(),
            file_path: None,
        };

        if let Some(path) = path {
            repo.file_path = Some(PathBuf::from(path));
            repo.load_items()?;
        }

        Ok(repo)
    }

    pub fn delete(&mut self, id: usize) {
        self.items.retain(|item| item.id != id)
    }

    pub fn publish(self) -> Result<()> {
        if self.file_path.is_some() {
            let buf = serde_json::to_string(&self.items)?;
            let path = self.get_todo_file_path();
            return fs::write(path, buf)
        }

        Ok(())
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

    fn load_items(&mut self) -> Result<()> {
        let item_json = fs::read_to_string(self.get_todo_file_path())?;
        self.items = serde_json::from_str(&item_json)?;
        Ok(())
    }

    fn get_todo_file_path(&self) -> String {
        // unwrapping because if this were to fail then there's something _really_ wrong with the users setup...
        let home = home_dir().unwrap();
        // unwrapping because we only call this if we already know we have a filepath
        let path = self.file_path.as_ref().unwrap().display();
        format!("{}/{}", home.display(), path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_instantiatable() {
        let repository = ItemRepository::new(None);
        assert!(repository.is_ok())
    }
}
