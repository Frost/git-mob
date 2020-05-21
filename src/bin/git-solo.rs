use std::fs::File;
use std::process;
use git2::{Config, Repository};

fn main() {
    println!("{}", get_main_author());

    match Repository::open_from_env() {
        Ok(repo) => {
            truncate_gitmessage_template(repo);
        }
        Err(_e) => {
            eprintln!("Not in a git repository");
            process::exit(1);
        }
    }
}

fn get_main_author() -> String {
    let cfg = Config::open_default().unwrap();
    let name = cfg.get_entry("user.name").unwrap();
    let name = name.value().unwrap();
    let email = cfg.get_entry("user.email").unwrap();
    let email = email.value().unwrap();

    format!("{} <{}>", name, email)
}

fn truncate_gitmessage_template(repo: Repository) {
    let template_path = repo.path().join(".gitmessage");
    let _template = File::create(template_path);
}
