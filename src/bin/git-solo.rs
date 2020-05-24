use std::fs::File;
use git_mob::{get_main_author, with_gitmessage_template_path_or_exit};

fn main() {
    let main_author = get_main_author();
    println!("{}", main_author);

    with_gitmessage_template_path_or_exit(|path| {
        let _template = File::create(path);
    })
}
