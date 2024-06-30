use std::process;

use crate::action::Action;

pub struct Add {}

impl Action for Add {
    fn usage(&self, name: &str, code: i32)
    {
        println!("usage: {} add URL KEYWORD1 [KEYWORD2 ...]", name);
        process::exit(code);
    }
    fn handle(&self, args: &Vec<String>) -> Result<(), &str>
    {
        if args.len() < 2 {
            return Err("you must provide a URL and at least one keyword!")
        }
        Ok(())
    }
}
