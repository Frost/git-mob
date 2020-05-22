use git_mob::{Author, get_main_author, get_available_coauthors};
use structopt::StructOpt;
use std::process;
use std::collections::LinkedList;

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
    } else {
        let mut coauthors = select_coauthors(&opt.coauthors);
        coauthors.push_front(get_main_author());

        for coauthor in coauthors {
            println!("{}", coauthor);
        }
    }
}

fn list_coauthors() {
    for (abbrev, author) in &get_available_coauthors() {
        println!("{}\t{}", abbrev, author);
    }
}

fn select_coauthors(coauthor_initials : &[String]) -> LinkedList<Author> {
    let all_coauthors = get_available_coauthors();
    let mut coauthors : LinkedList<Author> = LinkedList::new();

    for initial in coauthor_initials {
        match all_coauthors.get(initial) {
            Some(coauthor) => coauthors.push_back(coauthor.clone()),
            None => {
                eprintln!("Error: atuhor with initials {} not found", initial);
                process::exit(1);
            }
        }
    }

    coauthors
}
