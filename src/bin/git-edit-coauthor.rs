use clap::Parser;
use git_mob::{cli, get_available_coauthors, write_coauthors_file, Author};
use std::process;

fn main() {
    let opt = cli::GitDeleteCoauthor::parse();

    let mut authors = get_available_coauthors();

    if let Some(author) = authors.get(&opt.initials) {
        let mut updated_author: Author = author.clone();
        updated_author.name = opt.name;
        updated_author.email = opt.email;

        authors.insert(opt.initials, updated_author);

        write_coauthors_file(authors);
    } else {
        eprintln!("No author found with initials {}", &opt.initials);
        process::exit(1);
    };
}
