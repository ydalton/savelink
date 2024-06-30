use std::{env, process};

mod link;
mod search;
mod add;
mod action;

use add::Add;
use search::Search;
use action::Action;

fn usage(name: &str, code: i32) 
{
    println!("usage: {} ARG1 [ARG2]", name);
    process::exit(code);
}

fn get_action(arg: &str) -> Option<Box<dyn Action>>
{
    match arg {
        "add" => {
            return Some(Box::new(Add {}))
        },
        "search" => {
            return Some(Box::new(Search {}))
        }
        _ => return None,
    };
}

fn parse_cmd_line(args: &Vec<String>)
{

    if args.len() < 2 {
        usage(&args[0], 1);
    }

    let invoked_name = &args[0][..];
    let args_cloned = &args[2..].to_vec();
    let action = match get_action(&args[1][..]) {
        Some(act) => act,
        None => {
            usage(invoked_name, 1);
            return;
        },
    };
    if let Err(e) = action.handle(args_cloned) {
        println!("error: {}", e);
        action.usage(invoked_name, 1);
    }
}

fn main() 
{
    let args: Vec<String> = env::args().collect();

    parse_cmd_line(&args);
}
