use clap::{Parser, ValueEnum};
use fancy::printcoln;
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
    #[clap(value_enum, default_value_t = Action::List)]
    action: Action,
    #[arg(short, long, default_value_t = String::from("none"))]
    description: String,
    #[arg(short, long, default_value_t = 0)]
    index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
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
        println!("----------");
        for (i, todo) in self.todo_list.iter().enumerate() {
            if todo.done {
                printcoln!("[s]{}. {}[:]", i, todo.description);
            } else {
                printcoln!("{}. {}", i, todo.description);
            }
        }
    }

    fn add_todo(&mut self, description: String) {
        let new = Todo {
            done: false,
            description,
        };
        self.todo_list.push(new);
    }

    fn remove_todo(&mut self, index: usize) {
        self.todo_list.remove(index);
    }

    fn done_todo(&mut self, index: usize) {
        for (i, todo) in self.todo_list.iter_mut().enumerate() {
            if i == index {
                todo.done = true;
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
        Action::Add => db.add_todo(args.description),
        Action::Remove => db.remove_todo(args.index),
        Action::Done => db.done_todo(args.index),
        Action::List => (),
    }
    db.display();
    let serialized = serde_json::to_string_pretty(&db).unwrap();
    fs::write("./todozist.json", serialized).expect("fail to write json");
}
