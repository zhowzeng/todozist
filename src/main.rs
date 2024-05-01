use clap::{Parser, ValueEnum};
use std::fs;
use todozist::{Database, Todo};

#[derive(ValueEnum, Clone, Debug)]
enum Action {
    List,
    Add,
    Done,
    Remove,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(value_enum, default_value_t = Action::List)]
    action: Action,
    #[arg(short, long, default_value_t = String::from(""))]
    description: String,
    #[arg(short, long, default_values_t = vec![0], value_delimiter = ' ', num_args = 1..)]
    indices: Vec<usize>,
}

fn main() {
    let args = Args::parse();
    let mut db = match fs::read_to_string("./todozist.json") {
        Ok(s) => {
            println!("> load database...");
            let deserialized: Database = serde_json::from_str(&s).expect("fail to parse database");
            deserialized
        }
        Err(_) => {
            println!("> create new database...");
            let todo_list: Vec<Todo> = Vec::new();
            Database::new(todo_list)
        }
    };
    match args.action {
        Action::Add => db.add_todo(args.description),
        Action::Remove => db.remove_todo(args.indices),
        Action::Done => db.done_todo(args.indices),
        Action::List => (),
    }
    db.display();
    let serialized = serde_json::to_string_pretty(&db).unwrap();
    fs::write("./todozist.json", serialized).expect("fail to write json");
}
