use std::{env, process};

mod action;
mod link;

use action::{Action, Add, Open, Search};

fn usage(name: &str, code: i32) {
    println!("usage: {} COMMAND ...\n", name);
    println!("Commands:");
    println!("  add\t\tAdd a link to the database");
    println!("  search\tRetrieve a link from the database");
    println!("  open\t\tSame as search, but opens the link in the browser");
    process::exit(code);
}

fn get_action(arg: &str) -> Option<Box<dyn Action>> {
    match arg {
        "add" => return Some(Box::new(Add {})),
        "search" => return Some(Box::new(Search {})),
        "open" => return Some(Box::new(Open {})),
        _ => return None,
    };
}

fn parse_cmd_line(args: &Vec<String>) {
    if args.len() < 2 || args.iter().find(|a| *a == "--help").is_some() {
        usage(&args[0], 1);
    }

    let invoked_name = &args[0][..];
    let action = match get_action(&args[1][..]) {
        Some(act) => act,
        None => {
            usage(invoked_name, 1);
            return;
        }
    };
    let args_cloned = &args[2..].to_vec();
    if let Err(e) = action.handle(args_cloned) {
        println!("error: {}\n", e);
        println!("{}", action.usage(invoked_name));
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    parse_cmd_line(&args);
}
