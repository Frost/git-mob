use git_mob::{get_available_coauthors, write_coauthors_file};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Initials of the co-author to delete
    initials: String,
}

fn main() {
    let opt = Opt::from_args();
    let mut authors = get_available_coauthors();
    authors.remove(&opt.initials);
    write_coauthors_file(authors);
}
