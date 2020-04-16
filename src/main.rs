pub mod api {
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ToDoList<T> {
        pub items: Vec<T>,
        pub name: String,
    }

    #[derive(Debug)]
    pub enum Instruction {
        Add(String),
        Remove(usize),
        Modify(usize, String),
        Print,
    }
}

pub mod cli {
    use super::api::*;

    // use clap::{load_yaml, App};
    use clap::clap_app;

    pub fn parse() -> Option<Instruction> {
        let matches = clap_app!(todo_cli =>
            (version: "0.1")
            (author: "USACS at Rutgers University <usacs.rutgers.edu>")
            (about: "Simple to-do list CLI in Rust")
            (@arg print: -p --print "Print out all valued stored in CLI")
            (@subcommand add =>
                (@arg NEW: +required +takes_value "Task to add")
                (about: "Add a task to CLI")
            )
            (@subcommand rm =>
                (@arg NUM: +required +takes_value "Identifier of task to remove")
                (about: "Remove a task from CLI")
            )
            (@subcommand modify =>
                (@arg NUM: +required +takes_value "Identifier of task to modify")
                (@arg NEW: -n --new +required +takes_value "Task number to modify")
                (about: "Modify a task stored by the CLI")
            )
        )
        .get_matches();

        if let Some(matches) = matches.subcommand_matches("add") {
            return Some(Instruction::Add(matches.value_of("NEW")?.to_string()));
        } else if let Some(matches) = matches.subcommand_matches("rm") {
            return Some(Instruction::Remove(
                matches.value_of("NUM")?.parse().unwrap(),
            ));
        } else if let Some(matches) = matches.subcommand_matches("modify") {
            return Some(Instruction::Modify(
                matches.value_of("NUM")?.parse().unwrap(),
                matches.value_of("NEW")?.to_string(),
            ));
        } else if let Some(_) = matches.value_of("print") {
            return Some(Instruction::Print);
        }

        return None;
    }
}

fn main() {
    // TODO: Build persistence layer
}
