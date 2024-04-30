use serde::{Deserialize, Serialize};
use std::fs;

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
        for todo in self.todo_list.iter() {
            if todo.done {
                println!("[x] {}", todo.description);
            } else {
                println!("[ ] {}", todo.description);
            }
        }
    }
}

fn main() {
    let mut db = match fs::read_to_string("./todozist.json") {
        Ok(s) => {
            let deserialized: Database = serde_json::from_str(&s).expect("fail to parse database");
            deserialized
        }
        Err(_) => {
            let list: Vec<Todo> = Vec::new();
            Database {
                owner: String::from("zhow"),
                todo_list: list,
            }
        }
    };
    // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    let todo = Todo {
        done: false,
        description: String::from("do something"),
    };
    let todo2 = Todo {
        done: true,
        description: String::from("done something"),
    };
    db.todo_list.push(todo);
    db.todo_list.push(todo2);
    db.display();
    let serialized = serde_json::to_string_pretty(&db).unwrap();
    fs::write("./todozist.json", serialized).expect("fail to write json");
}
