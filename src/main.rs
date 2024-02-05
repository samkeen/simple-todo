use std::env;
use std::process::exit;
use todo_collection::TodoCollection;

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
    let mut persist_file_path = env::current_dir().expect("Failed to read current directory");
    persist_file_path.push("todo.json");
    let persist_file_path = persist_file_path
        .to_str()
        .expect("Failed to derive the absolute path to the todo.json file");
    let mut todos = TodoCollection::new(persist_file_path);

    match command {
        "add" => match args.get(2) {
            Some(value) => todos.add(value),
            None => println!(
                "The add command requires something to add.\nUSAGE: $ todo add \"Clean room\""
            ),
        },
        "list" => todos.list(),
        "done" => match args.get(2) {
            Some(value) => todos.remove(value),
            None => println!(
                "The done command requires the identifier of the todo.\nUSAGE: $ todo done <id>"
            ),
        },
        unknown => println!("Unknown command: {}", unknown),
    }
}
