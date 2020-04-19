pub mod cli {
    use crate::api::api::Instruction;
    use clap::clap_app;

    pub fn parse() -> Option<Instruction<String>> {
        let matches = clap_app!(todo_cli =>
            (version: "0.1")
            (author: "USACS at Rutgers University <usacs.rutgers.edu>")
            (about: "Simple to-do list CLI in Rust")
            (@subcommand print =>
                (about: "Print out all values stored in CLI")
            )
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
	    // Why can't we use a `?` after the parse?
            return if let Ok(idx) = matches.value_of("NUM")?.parse() {
		Some(Instruction::Remove(idx))
            } else {
		None
	    }
        } else if let Some(matches) = matches.subcommand_matches("modify") {
            return Some(Instruction::Modify(
		// This code might panic. Why? Exercise(Week 1): gracefully handle
		// the error case.
                matches.value_of("NUM")?.parse().unwrap(),
                matches.value_of("NEW")?.to_string(),
            ));
        } else if matches.is_present("print") {
            return Some(Instruction::Print);
        }

        return None;
    }
}
