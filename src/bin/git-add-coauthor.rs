use clap::Parser;
use git_mob::{parse_coauthors_file, write_coauthors_file, Author};

#[derive(Parser, Debug)]
#[clap(name = "git-add-coauthor", version)]
/// Add a co-author to your git mob.
struct Opt {
    /// Co-author initials
    initials: String,
    /// The name of the co-author, in quotes, e.g. "Foo Bar"
    name: String,
    /// The email of the co-author
    email: String,
}

fn main() {
    let opt = Opt::parse();
    let mut authors = parse_coauthors_file().unwrap_or_default();
    let new_author = Author {
        name: opt.name,
        email: opt.email,
    };

    authors.insert(opt.initials, new_author);

    write_coauthors_file(authors);
}
