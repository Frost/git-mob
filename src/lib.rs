use std::fs::File;
use git2::{Config, Repository};

pub fn get_main_author() -> Author {
    let cfg = Config::open_default().unwrap();
    let name = cfg.get_entry("user.name").unwrap();
    let email = cfg.get_entry("user.email").unwrap();

    Author {
        name: name.value().unwrap().to_string(),
        email: email.value().unwrap().to_string(),
    }
}
