pub mod api {
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;

    // You do not need this in week 1.
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ToDoList<T> {
        pub items: Vec<T>,
        pub name: String,
    }

    // Week 1: you do not need the "#[derive..." line.
    // It will be explained in the later weeks.
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
	    // Why can't we use `?` after the parse?
            return if let Ok(n) = matches.value_of("NUM")?.parse() {
		Some(Instruction::Remove(n));
	    } else {
		None
	    }
        } else if let Some(matches) = matches.subcommand_matches("modify") {
            return Some(Instruction::Modify(
		// Exercise (week 1): this is unsafe since if the user tries to, persay,
		// $ ./todo-cli modify dog --new "feed the dog"
		// the code will panic. How might you fix this?
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
    println!(cli::parse())
}
