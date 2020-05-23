use git_mob::{Author, get_main_author, get_available_coauthors, gitmessage_template_file_path, with_repo_or_exit};
use git2::Repository;
use structopt::StructOpt;
use std::process;
use std::fs;

#[derive(StructOpt,Debug)]
struct Opt {
    /// Prints list of available co-authors
    #[structopt(short,long)]
    list: bool,
    /// A list of co-author initials
    coauthors: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();

    if opt.list {
        list_coauthors();
        process::exit(0);
    }

    write_coauthors_to_gitmessage_file(&opt.coauthors);
}

fn list_coauthors() {
    for (abbrev, author) in &get_available_coauthors() {
        println!("{}\t{}", abbrev, author);
    }
}

fn write_coauthors_to_gitmessage_file(coauthor_initials: &[String]) {
    let coauthors = select_coauthors(&coauthor_initials);
    let mut content = String::from("\n\n");
    for author in &coauthors {
        content.push_str(&format!("Co-authored-by: {}\n", &author.to_string()));
    }

    with_repo_or_exit(|repo: Repository| {
        let path = gitmessage_template_file_path(repo);

        match fs::write(path, content) {
            Ok(_) => {
                println!("{}", get_main_author());
                for author in &coauthors {
                    println!("{}", author);
                }
            },
            Err(e) => {
                eprintln!("Error writing to .gitmessage template: {}", e);
                process::exit(1);
            },
        }
    });
}

fn select_coauthors(coauthor_initials : &[String]) -> Vec<Author> {
    let all_coauthors = get_available_coauthors();
    let mut coauthors : Vec<Author> = Vec::new();

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
