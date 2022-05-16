use clap::Parser;
use git_mob::{get_available_coauthors, write_coauthors_file, Author};
use std::process;

#[derive(Parser, Debug)]
#[clap(name = "git-edit-coauthor", version)]
/// Edit a co-author in your .git-coauthors template
struct Opt {
    /// Co-author initials
    initials: String,
    /// The name of the co-author, in quotes, e.g. "Foo Bar"
    #[clap(long, required_unless_present("email"))]
    name: Option<String>,
    /// The email of the co-author
    #[clap(long, required_unless_present("name"))]
    email: Option<String>,
}

fn main() {
    let opt = Opt::parse();

    let mut authors = get_available_coauthors();

    if let Some(author) = authors.get(&opt.initials) {
        let mut updated_author: Author = author.clone();
        if let Some(name) = opt.name {
            updated_author.name = name;
        };

        if let Some(email) = opt.email {
            updated_author.email = email;
        };

        authors.insert(opt.initials, updated_author);

        write_coauthors_file(authors);
    } else {
        eprintln!("No author found with initials {}", &opt.initials);
        process::exit(1);
    };
}
