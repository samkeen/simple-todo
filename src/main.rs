use chrono::Utc;
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::process::exit;
use std::{env, fs};

const PERSIST_FILE_PATH: &str = "/todo.json";

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

fn get_existing_todos() -> Vec<Todo> {
    /*
      This next line is equivalent to this match expression:
        let existing_todos_serialized = match fs::read_to_string(PERSIST_FILE_PATH) {
            Ok(content) => content,
            Err(_) => "[]".to_string(),
        };
    */
    let existing_todos_serialized =
        fs::read_to_string(PERSIST_FILE_PATH).unwrap_or_else(|_| "[]".to_string());
    return serde_json::from_str(&existing_todos_serialized).unwrap_or_else(|_| Vec::new());
}
fn persist_todos(todos: Vec<Todo>) {
    let serialized = serde_json::to_string(&todos).expect("Failed to serialize data");
    fs::write(PERSIST_FILE_PATH, serialized).expect("Could not write to file");
}

fn add_todo(text: &str) {
    let todo = Todo {
        id: generate_id(),
        text: text.to_string(),
        is_complete: false,
    };
    let todo_id = todo.id.clone();
    let mut existing_todos = get_existing_todos();
    existing_todos.push(todo);
    persist_todos(existing_todos);
    println!("Added Todo[{}] '{}'", todo_id, text);
}

fn list_todos() {
    println!("== Existing todos ==");
    let existing_todos = get_existing_todos();
    if existing_todos.len() == 0 {
        println!("There are no saved Todos");
        exit(0)
    }
    for todo in existing_todos {
        println!("[{}] {}", todo.id, todo.text)
    }
}

fn remove_todo(todo_id: &str) {
    let mut existing_todos = get_existing_todos();
    match existing_todos.iter().find(|&todo| todo.id == todo_id) {
        Some(todo_to_remove) => {
            println!(
                "Removed Todo[{}] '{}'",
                todo_to_remove.id, todo_to_remove.text
            );
            existing_todos.retain(|todo| todo.id != todo_id);
            persist_todos(existing_todos);
        }
        None => {
            println!("Todo with id [{}] was not found", todo_id)
        }
    };
}

fn main() {
    // read the commandline args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No command given");
        println!("USAGE: todo <command> [arg]");
        println!("Known commands:");
        println!("todo add \"todo description\"");
        println!("todo list");
        println!("done <id of todo>");
        exit(0)
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
