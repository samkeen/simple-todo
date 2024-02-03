use chrono::Utc;
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::{Result, Write};
use std::{env, fs};

fn generate_id() -> String {
    let current_time = Utc::now().to_string();
    let mut hasher = Sha256::new();
    hasher.update(current_time);
    let result = hasher.finalize();
    let hex_string = hex::encode(result);
    hex_string[..6].to_string()
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: String,
    text: String,
    is_complete: bool,
}

fn get_existing_todos() -> Result<Vec<Todo>> {
    let file_content = fs::read_to_string("../todo.json").expect("Could not read file");
    let todos: Vec<Todo> = serde_json::from_str(&file_content)?;
    Ok(todos)
}
fn persist_todos(todos: Vec<Todo>) {
    let serialized = serde_json::to_string(&todos).expect("Failed to serialize data");
    fs::write("../todo.json", serialized).expect("Could not write to file");
}

fn add_todo(text: &str) {
    let mut todo = Todo {
        id: generate_id(),
        text: text.to_string(),
        is_complete: false,
    };
    let mut existing_todos = match get_existing_todos() {
        Ok(todos) => todos,
        Err(e) => panic!("failed to retrieve existing todos: {}", e),
    };
    existing_todos.push(todo);
    persist_todos(existing_todos)
}

fn list_todos() {}

fn remove_todo(todo_id: &str) {}

fn main() {
    // read the commandline args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("USAGE: no command given")
    }

    let command = args[1].as_str();

    match command {
        "add" => match args.get(2) {
            Some(value) => add_todo(value),
            None => println!(
                "The add command requires something to add.\nUSAGE: $ todo add \"Clean room\""
            ),
        },
        "list" => list_todos(),
        "done" => match args.get(2) {
            Some(value) => remove_todo(value),
            None => println!(
                "The done command requires the identifier of the todo.\nUSAGE: $ todo done <id>"
            ),
        },
        unknown => println!("Unknown command: {}", unknown),
    }
}
