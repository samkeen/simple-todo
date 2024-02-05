use chrono::Utc;
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: String,
    text: String,
    is_complete: bool,
}

#[derive(Debug)]
pub struct TodoCollection {
    todos: Vec<Todo>,
    persist_file_path: String
}

impl TodoCollection {
    pub fn new(persist_file_path: &str) -> Self {
        TodoCollection {
            todos: TodoCollection::get_existing_todos(&persist_file_path),
            persist_file_path: persist_file_path.to_string()
        }
    }

    pub fn add(&mut self, text: &str) {
        let todo = Todo {
            id: TodoCollection::generate_id(),
            text: text.to_string(),
            is_complete: false,
        };
        let todo_id = todo.id.clone();
        self.todos.push(todo);
        self.persist();
        println!("Added Todo[{}] '{}'", todo_id, text);
    }

    pub fn list(&self) {
        println!("== Existing todos ==");
        if self.todos.len() == 0 {
            println!("There are no saved Todos");
        } else {
            for todo in &self.todos {
                println!("[{}] {}", todo.id, todo.text)
            }
        }
    }

    pub fn remove(&mut self, todo_id: &str) {
        match self.todos.iter().find(|&todo| todo.id == todo_id) {
            Some(todo_to_remove) => {
                println!(
                    "Removed Todo[{}] '{}'",
                    todo_to_remove.id, todo_to_remove.text
                );
                self.todos.retain(|todo| todo.id != todo_id);
                self.persist();
            }
            None => {
                println!("Todo with id [{}] was not found", todo_id)
            }
        };
    }

    fn get_existing_todos(persist_file_path: &str) -> Vec<Todo> {
        let existing_todos_serialized =
            fs::read_to_string(persist_file_path).unwrap_or_else(|_| "[]".to_string());
        return serde_json::from_str(&existing_todos_serialized).unwrap_or_else(|_| Vec::new());
    }

    fn generate_id() -> String {
        let mut hasher = Sha256::new();
        hasher.update(Utc::now().to_string());
        hex::encode(hasher.finalize())[..6].to_string()
    }
    fn persist(&self) {
        let serialized = serde_json::to_string(&self.todos).expect("Failed to serialize data");
        fs::write(&self.persist_file_path, serialized).expect("Could not write to file");
    }
}
