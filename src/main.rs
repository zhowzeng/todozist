use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;

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
    // #[arg(short, long)]
    #[clap(value_enum)]
    action: Action,
    #[arg(short, long)]
    id: u8,
    #[arg(short, long)]
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u8,
    done: bool,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Database {
    owner: String,
    todo_list: Vec<Todo>,
}

impl Database {
    fn display(&self) {
        println!("owner: {}", self.owner);
        for todo in self.todo_list.iter() {
            if todo.done {
                println!("[x] {}) {}", todo.id, todo.description);
            } else {
                println!("[ ] {}) {}", todo.id, todo.description);
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    let mut db = match fs::read_to_string("./todozist.json") {
        Ok(s) => {
            println!("load database");
            let deserialized: Database = serde_json::from_str(&s).expect("fail to parse database");
            deserialized
        }
        Err(_) => {
            println!("create new database");
            let list: Vec<Todo> = Vec::new();
            Database {
                owner: String::from("zhow"),
                todo_list: list,
            }
        }
    };
    match args.action {
        Action::Add => {
            let new = Todo {
                id: args.id,
                done: false,
                description: args.description,
            };
            db.todo_list.push(new);
        }
        Action::Remove => {
            db.todo_list.retain(|todo| todo.id != args.id);
        }
        Action::Done => {
            for todo in db.todo_list.iter_mut() {
                if todo.id == args.id {
                    todo.done = true;
                }
            }
        }
        _ => {}
    }
    db.display();
    let serialized = serde_json::to_string_pretty(&db).unwrap();
    fs::write("./todozist.json", serialized).expect("fail to write json");
}
