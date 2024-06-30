use std::process;

use crate::Action;

pub struct Search {}

impl Action for Search {
    fn usage(&self, name: &str, code: i32)
    {
        println!("usage: {} search KEYWORD1 [KEYWORD2 ...]", name);
        process::exit(code);
    }
    fn handle(&self, args: &Vec<String>) -> Result<(), &str>
    {
        if args.len() < 1 {
            return Err("you must provide at least one keyword!");
        }

        Ok(())
    }
}
