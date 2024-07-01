use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write, Seek};

use serde_json::{to_string, from_str};

use crate::link::Link;

const DB_PATH: &str = "savelink_db.json";

pub fn add_to_file(link: Link) {
    let mut path = PathBuf::new();
    let mut contents: String = String::new();
    let mut link_list: Vec<Link>;

    path.push(option_env!("HOME").unwrap());
    path.push(".config");
    path.push(DB_PATH);

    let mut file = File::options()
                    .write(true)
                    .read(true)
                    .create(true)
                    .open(path)
                    .unwrap();

    file.read_to_string(&mut contents).unwrap();

    link_list = if contents.is_empty() {
        Vec::new()
    } else {
        from_str(&contents)
                .expect("Failed to deserialize file contents!")
    };

    file.set_len(0).expect("Failed to truncate file!");
    file.rewind().unwrap();

    link_list.push(link);

    let serialized = to_string(&link_list)
                            .expect("Failed to serialize link list!");

    dbg!(&serialized);

    file.write_all(serialized.as_bytes())
                        .expect("Failed to write to file!");

}
