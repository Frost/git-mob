use git_mob::{get_available_coauthors, write_coauthors_file, Author};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Co-author initials
    initials: String,
    /// The name of the co-author, in quotes, e.g. "Foo Bar"
    name: String,
    /// The email of the co-author
    email: String,
}

fn main() {
    let opt = Opt::from_args();
    let mut authors = get_available_coauthors();
    let new_author = Author {
        name: opt.name,
        email: opt.email,
    };
    authors.insert(opt.initials, new_author);

    write_coauthors_file(authors);
}
