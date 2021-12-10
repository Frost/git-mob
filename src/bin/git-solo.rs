use clap::Parser;
use git_mob::{
    ensure_commit_template_is_set, get_main_author, with_gitmessage_template_path_or_exit,
};
use std::fs::File;

#[derive(Parser, Debug)]
#[clap(name = "git-solo", version)]
/// Disband the mob and continue working solo.
struct Opt {}

fn main() {
    let _opt = Opt::parse();
    let main_author = get_main_author();
    println!("{}", main_author);

    with_gitmessage_template_path_or_exit(|path| {
        let _template = File::create(path);
    });
    ensure_commit_template_is_set();
}
