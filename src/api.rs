use std::fs;
use std::ops::Drop;

use prettytable::*;
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::fmt::{Debug, Display};

pub static BACKUP_FILE: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDoList<T>
where
    T: Display + Serialize,
{
    pub tasks: Vec<T>,
    name: String,
}

impl<T: Display + Serialize> ToDoList<T> {
    pub fn new(name: String) -> ToDoList<T> {
        ToDoList {
            tasks: Vec::new(),
            name,
        }
    }

    pub fn run(&mut self, inst: Instruction<T>) {
        match inst {
            Instruction::Add(t) => self.tasks.push(t),
            Instruction::Modify(i, t) => self.tasks[i - 1] = t,
            Instruction::Remove(i) => {
                self.tasks.remove(i - 1);
            }
            Instruction::Print => {
                if !self.tasks.is_empty() {
                    let mut table = Table::new();

                    for (i, s) in self.tasks.iter().enumerate() {
                        table.add_row(row![(i + 1).to_string(), s]);
                    }

                    println!("\n\n{}'s To-Do List:\n", self.name);
                    table.printstd();
                } else {
                    println!("No tasks to print for {}", self.name);
                }
            }
        }
    }
}

impl<T: Display + Serialize> Drop for ToDoList<T> {
    fn drop(&mut self) {
        let userdata = json::to_string_pretty(self).unwrap();
        fs::write(BACKUP_FILE, userdata).unwrap();
    }
}

#[derive(Debug)]
pub enum Instruction<T> {
    Add(T),
    Remove(usize),
    Modify(usize, T),
    Print,
}

#[cfg(test)]
mod tests {
    use super::{Instruction, ToDoList};

    #[test]
    fn test_to_do_list() {
        let mut list: ToDoList<String> = ToDoList::new("Hari".to_string());

        list.run(Instruction::Add("Write Rust tutorial".to_string()));
        assert_eq!(list.tasks[0], "Write Rust tutorial");

        list.run(Instruction::Modify(
            1,
            "Make fun of languages that aren't Rust".to_string(),
        ));
        assert_eq!(list.tasks[0], "Make fun of languages that aren't Rust");

        list.run(Instruction::Remove(1));
        assert!(list.tasks.is_empty());
    }
}
