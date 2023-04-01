use clap::Parser;
use git_mob::{cli, parse_coauthors_file, write_coauthors_file, Author};

fn main() {
    let opt = cli::GitAddCoauthor::parse();
    let mut authors = parse_coauthors_file().unwrap_or_default();
    let new_author = Author {
        name: opt.name,
        email: opt.email,
    };

    authors.insert(opt.initials, new_author);

    write_coauthors_file(authors);
}
