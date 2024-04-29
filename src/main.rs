use serde::{Deserialize, Serialize};

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
    let mut list: Vec<Todo> = Vec::new();
    let todo = Todo {
        done: false,
        description: String::from("do something"),
    };
    let todo2 = Todo {
        done: true,
        description: String::from("done something"),
    };
    list.push(todo);
    list.push(todo2);
    let db = Database {
        owner: String::from("zhow"),
        todo_list: list,
    };
    db.display();
    let serialized = serde_json::to_string(&db).unwrap();
    println!("{}", serialized);
    // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
}
