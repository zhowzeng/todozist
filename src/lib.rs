use fancy::printcoln;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    done: bool,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    owner: String,
    todo_list: Vec<Todo>,
}

impl Database {
    pub fn new(todo_list: Vec<Todo>) -> Database {
        Database { owner: String::from("zhow"), todo_list }
    }
    pub fn display(&self) {
        println!("");
        printcoln!("[b]Todo Zist[:]");
        println!("");
        for (i, todo) in self.todo_list.iter().enumerate() {
            if todo.done {
                printcoln!("[s]{}. {}", i, todo.description);
            } else {
                printcoln!("{}. {}", i, todo.description);
            }
        }
        println!("");
    }

    pub fn add_todo(&mut self, description: String) {
        let new = Todo {
            done: false,
            description,
        };
        self.todo_list.push(new);
    }

    pub fn remove_todo(&mut self, indices: Vec<usize>) {
        // self.todo_list.remove(index);
        let mut new: Vec<Todo> = Vec::new();
        for (i, todo) in self.todo_list.iter_mut().enumerate() {
            if !indices.contains(&i) {
                new.push(todo.clone())
            }
        }
        self.todo_list = new;
    }

    pub fn done_todo(&mut self, indices: Vec<usize>) {
        for (i, todo) in self.todo_list.iter_mut().enumerate() {
            if indices.contains(&i) {
                todo.done = true;
            }
        }
    }
}
