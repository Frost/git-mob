use git_mob::{
    ensure_commit_template_is_set, get_available_coauthors, get_main_author, set_main_author,
    with_gitmessage_template_path_or_exit, Author,
};
use std::fs;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Prints list of available co-authors
    #[structopt(short, long)]
    list: bool,
    /// Overwrite the main author
    #[structopt(short, long)]
    overwrite: Option<String>,
    /// A list of co-author initials
    coauthors: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();

    if opt.list {
        list_coauthors();
        process::exit(0);
    }

    if let Some(initials) = opt.overwrite {
        override_main_author(&initials);
    }

    write_coauthors_to_gitmessage_file(&opt.coauthors);
    ensure_commit_template_is_set();
}

fn list_coauthors() {
    for (abbrev, author) in &get_available_coauthors() {
        println!("{}\t{}", abbrev, author);
    }
}

fn override_main_author(initials: &String) {
    let all_authors = get_available_coauthors();
    match all_authors.get(initials) {
        Some(new_main_author) => set_main_author(&new_main_author),
        None => {
            eprintln!("Error: author with initials {} not found", initials);
            process::exit(1);
        }
    }
}

fn write_coauthors_to_gitmessage_file(coauthor_initials: &[String]) {
    let coauthors = select_coauthors(&coauthor_initials);
    let mut content = String::from("\n\n");
    for author in &coauthors {
        content.push_str(&format!("Co-authored-by: {}\n", &author.to_string()));
    }

    with_gitmessage_template_path_or_exit(|path| match fs::write(path, content) {
        Ok(_) => {
            println!("{}", get_main_author());
            for author in &coauthors {
                println!("{}", author);
            }
        }
        Err(e) => {
            eprintln!("Error writing to .gitmessage template: {}", e);
            process::exit(1);
        }
    });
}

fn select_coauthors(coauthor_initials: &[String]) -> Vec<Author> {
    let all_coauthors = get_available_coauthors();
    let mut coauthors: Vec<Author> = Vec::new();

    for initial in coauthor_initials {
        match all_coauthors.get(initial) {
            Some(coauthor) => coauthors.push(coauthor.clone()),
            None => {
                eprintln!("Error: atuhor with initials {} not found", initial);
                process::exit(1);
            }
        }
    }

    coauthors
}
