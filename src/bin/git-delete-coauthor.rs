use clap::Parser;
use git_mob::{get_available_coauthors, write_coauthors_file};

#[derive(Parser, Debug)]
#[clap(name = "git-delete-coauthor", version)]
/// Delete a co-author from your .git-coauthors file
struct Opt {
    /// Initials of the co-author to delete
    initials: String,
}

fn main() {
    let opt = Opt::parse();
    let mut authors = get_available_coauthors();
    authors.remove(&opt.initials);
    write_coauthors_file(authors);
}
