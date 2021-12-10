use git_mob::{get_available_coauthors, write_coauthors_file, Author};
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="git-edit-coauthor")]
/// Edit a co-author in your .git-coauthors template
struct Opt {
    /// Co-author initials
    initials: String,
    /// The name of the co-author, in quotes, e.g. "Foo Bar"
    #[structopt(long, required_unless("email"))]
    name: Option<String>,
    /// The email of the co-author
    #[structopt(long, required_unless("name"))]
    email: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let mut authors = get_available_coauthors();
    let mut updated_author: Author;

    if let Some(author) = authors.get(&opt.initials) {
        updated_author = author.clone();
    } else {
        eprintln!("No author found with initials {}", &opt.initials);
        process::exit(1);
    }

    if let Some(name) = opt.name {
        updated_author.name = name;
    }

    if let Some(email) = opt.email {
        updated_author.email = email;
    }

    authors.insert(opt.initials, updated_author);

    write_coauthors_file(authors);
}
