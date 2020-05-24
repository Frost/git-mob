use serde::{Deserialize, Serialize};
use std::fs;
use git2::{Config, Repository};
use std::fmt;
use std::collections::BTreeMap;
use dirs::home_dir;

use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use std::string::String;
use std::process;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Author {
    pub name: String,
    pub email: String,
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

pub fn get_main_author() -> Author {
    let cfg = Config::open_default().unwrap();
    let name = cfg.get_entry("user.name").unwrap();
    let email = cfg.get_entry("user.email").unwrap();

    Author {
        name: name.value().unwrap().to_string(),
        email: email.value().unwrap().to_string(),
    }
}

pub fn get_available_coauthors() -> BTreeMap<String, Author> {
    match parse_coauthors_file() {
        Ok(coauthors) => coauthors,
        Err(e) => {
            eprintln!("{}", e);
            BTreeMap::new()
        }
    }
}

fn parse_coauthors_file() -> Result<BTreeMap<String, Author>, Box<dyn Error>> {
    let coauthors_path = coauthors_file_path();
    let coauthors_file = File::open(coauthors_path)?;
    let reader = BufReader::new(coauthors_file);

    let json_data : serde_json::Value = serde_json::from_reader(reader)?;

    match json_data.get("coauthors") {
        Some(coauthors) => Ok(serde_json::from_value(coauthors.clone()).unwrap()),
        None => Ok(BTreeMap::new())
    }
}

pub fn with_gitmessage_template_path_or_exit<F: FnOnce(std::path::PathBuf)>(f: F) {
    match Repository::open_from_env() {
        Ok(repo) => {
            let path = repo.path().join(".gitmessage");
            f(path);
        },
        Err(_e) => {
            eprintln!("Not in a git repository");
            process::exit(1);
        }
    }
}

pub fn write_coauthors_file(authors: BTreeMap<String, Author>) {
    let mut wrapper_tree = BTreeMap::new();
    wrapper_tree.insert("coauthors", authors);
    let json_data = serde_json::to_string_pretty(&wrapper_tree).unwrap();
    match fs::write(coauthors_file_path(), json_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error writing git-coauthors file: {:?}", e);
            process::exit(1);
        }
    }
}

fn coauthors_file_path() -> std::path::PathBuf {
    home_dir().unwrap().join(".git-coauthors")
}
