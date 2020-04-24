use std::error::Error;
use std::fs::File;
use std::io::BufReader;

mod api;
mod cli;

use crate::api::{ToDoList, BACKUP_FILE};
use crate::cli::parse;

pub(crate) fn main() -> Result<(), Box<dyn Error>> {
    let mut list: ToDoList<String> = match File::open(BACKUP_FILE) {
        Ok(file) => {
            // file exists - deserialize and go with existing list
            let file = BufReader::new(file);
            serde_json::from_reader(file)?
        }
        Err(_) => {
            // file does not exist - make a new list
            ToDoList::new(env!("USER").to_string())
        }
    };

    Ok(list.run(parse()?))
    // match parse() {
    //     Some(inst) => list.run(inst),
    //     None => panic!("Arguments could not be parsed"),
    // }

    // Ok(())
}
