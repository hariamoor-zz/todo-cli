use crate::api::Instruction;
use std::error::Error;

use clap::clap_app;
use simple_error::bail;

pub fn parse() -> Result<Instruction<String>, Box<dyn Error>> {
    let matches = clap_app!(todo_cli =>
        (version: "0.1")
        (author: "USACS at Rutgers University")
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
        return Ok(Instruction::Add(
            matches
                .value_of("NEW")
                .expect("Need task to add")
                .to_string(),
        ));
    } else if let Some(matches) = matches.subcommand_matches("rm") {
        return Ok(Instruction::Remove(
            matches.value_of("NEW").expect("Need task to add").parse()?,
        ));
    } else if let Some(matches) = matches.subcommand_matches("modify") {
        return Ok(Instruction::Modify(
            // This code might panic. Why? Exercise(Week 1): gracefully handle
            // the error case.
            matches
                .value_of("NUM")
                .expect("Need index of task to modify".as_ref())
                .parse()?,
            matches
                .value_of("NEW")
                .expect("Need task to modify to")
                .to_string(),
        ));
    } else if matches.is_present("print") {
        return Ok(Instruction::Print);
    }

    bail!("Command-line arguments could not be parsed");
}
