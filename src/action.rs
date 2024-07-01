use crate::link::Link;
use url::Url;

pub trait Action {
    fn usage(&self, name: &str) -> String;
    fn handle(&self, args: &Vec<String>) -> Result<(), &str>;
}

pub struct Add {}
pub struct Search {}
pub struct Open {}

impl Action for Add {
    fn usage(&self, name: &str) -> String {
        format!("usage: {} add URL KEYWORD1 [KEYWORD2 ...]", name)
    }
    fn handle(&self, args: &Vec<String>) -> Result<(), &str> {
        if args.len() < 2 {
            return Err("you must provide a URL and at least one keyword!");
        }

        let url = &args[0];
        let keywords = &args[1..];

        if let Err(_) = Url::parse(&url) {
            return Err("first argument is not a url!");
        }

        let mut builder = Link::builder().link(url);

        for keyword in keywords {
            builder = builder.keyword(keyword);
        }

        let link = builder.build();

        dbg!(&link);
        Ok(())
    }
}

impl Action for Search {
    fn usage(&self, name: &str) -> String {
        format!("usage: {} search KEYWORD1 [KEYWORD2 ...]", name)
    }
    fn handle(&self, args: &Vec<String>) -> Result<(), &str> {
        if args.len() < 1 {
            return Err("you must provide at least one keyword!");
        }

        Ok(())
    }
}

impl Action for Open {
    fn usage(&self, name: &str) -> String {
        format!("usage: {} open KEYWORD1 [KEYWORD2 ...]", name)
    }
    fn handle(&self, _args: &Vec<String>) -> Result<(), &str> {
        unimplemented!();
    }
}
