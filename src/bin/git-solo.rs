use git2::Repository;
use std::fs::File;
use git_mob::{get_main_author, gitmessage_template_file_path, with_repo_or_exit};

fn main() {
    let main_author = get_main_author();
    println!("{}", main_author);

    with_repo_or_exit(truncate_gitmessage_template);
}

fn truncate_gitmessage_template(repo: Repository) {
    let path = gitmessage_template_file_path(repo);
    let _template = File::create(path);
}
