use clap::Parser;
use git_mob::{cli, get_available_coauthors, write_coauthors_file};

fn main() {
    let opt = cli::GitDeleteCoauthor::parse();
    let mut authors = get_available_coauthors();
    authors.remove(&opt.initials);
    write_coauthors_file(authors);
}
