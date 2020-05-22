use git2::Repository;
use std::process;
use std::fs::File;
use git_mob::{get_main_author, gitmessage_template_file_path};

fn main() {
    let main_author = get_main_author();
    println!("{}", main_author);

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

fn truncate_gitmessage_template(repo: Repository) {
    let path = gitmessage_template_file_path(repo);
    let _template = File::create(path);
}