use git_mob::cli;
use std::fmt::Write;
use std::fs;
use std::fs::File;
use std::process;
use git_mob::{
    ensure_commit_template_is_set, get_available_coauthors,
    get_main_author, set_main_author, with_gitmessage_template_path_or_exit,
    Author, parse_coauthors_file, write_coauthors_file
};

fn main() {
    let matches = cli::git_mob_cmd().get_matches();

    println!("matches: {:?}", matches);

    if let Some(handles) = matches.get_raw("with") {
        println!("with: {:?}", handles);
    }
    if matches.get_one("solo") == Some(&true) {
        solo();
    }
    if matches.get_one("current") == Some(&true) {
        println!("current.");
    }

    if let Some(add_args) = matches.get_raw("add") {
        println!("add: {:?}", add_args);

    }
    if let Some(edit_args) = matches.get_raw("edit") {
        println!("edit: {:?}", edit_args);

    }
    if let Some(delete_handle) = matches.get_one::<String>("delete") {
        println!("delete: {:?}", delete_handle);

    }
    if matches.get_one("list") == Some(&true) {
        list_coauthors();
    }



    // match matches.subcommand_name() {
    //     Some("add") => {
    //         let args = matches.subcommand_matches("add").unwrap();
    //         let handle = args.get_one("handle").unwrap();
    //         let name = args.get_one("name").unwrap();
    //         let email = args.get_one("email").unwrap();
    //         add_coauthor(handle, name, email);
    //     },
    //     Some("edit") => {
    //         let args = matches.subcommand_matches("edit").unwrap();
    //         let handle = args.get_one("handle").unwrap();
    //         let name = args.get_one("name").unwrap();
    //         let email = args.get_one("email").unwrap();
    //         edit_coauthor(handle, name, email);
    //     },
    //     Some("remove") => {
    //         let handle = matches.subcommand_matches("remove").unwrap()
    //             .get_one("handle").unwrap();
    //         remove_coauthor(handle);
    //     },
    //     Some("list") => list_coauthors(),
    //     Some("solo") => solo(),
    //     Some("with") => {
    //         let matches = matches.subcommand_matches("with").unwrap();
    //         let handles: Vec<String> =
    //             matches.get_many::<String>("handles").unwrap().map(|s| s.clone()).collect();

    //         if let Some(handle) = matches.get_one::<String>("override") {
    //             override_main_author(handle);
    //         }

    //         write_coauthors_to_gitmessage_file(&handles);
    //     },
    //     _ => println!("Something else"),
    // }
}

fn list_coauthors() {
    for (abbrev, author) in &get_available_coauthors() {
        println!("{abbrev}\t{author}");
    }
}

fn add_coauthor(handle: &String, name: &String, email: &String) {
    let mut authors = parse_coauthors_file().unwrap_or_default();
    let new_author = Author {
        name: name.to_string(),
        email: email.to_string(),
    };

    authors.insert(handle.to_string(), new_author);

    write_coauthors_file(authors);
}

fn edit_coauthor(handle: &String, name: &String, email: &String) {
    let mut authors = get_available_coauthors();

    if let Some(author) = authors.get(handle) {
        let mut updated_author: Author = author.clone();
        updated_author.name = name.to_string();
        updated_author.email = email.to_string();

        authors.insert(handle.to_string(), updated_author);

        write_coauthors_file(authors);
    } else {
        eprintln!("No author found with initials {}", handle);
        process::exit(1);
    };
}

fn remove_coauthor(handle: &String) {
    let mut authors = get_available_coauthors();
    authors.remove(handle);
    write_coauthors_file(authors);
}

fn solo() {
    let main_author = get_main_author();
    println!("{main_author}");

    with_gitmessage_template_path_or_exit(|path| {
        let _template = File::create(path);
    });
    ensure_commit_template_is_set();
}

fn override_main_author(initials: &str) {
    let all_authors = get_available_coauthors();
    match all_authors.get(initials) {
        Some(new_main_author) => set_main_author(new_main_author),
        None => {
            eprintln!("Error: author with initials {initials} not found");
            process::exit(1);
        }
    }
}

fn write_coauthors_to_gitmessage_file(coauthor_initials: &[String]) {
    let coauthors = select_coauthors(coauthor_initials);
    let mut content = String::from("\n\n");
    for author in &coauthors {
        _ = writeln!(content, "Co-authored-by: {}", &author.to_string());
    }

    with_gitmessage_template_path_or_exit(|path| match fs::write(path, content) {
        Ok(_) => {
            println!("{}", get_main_author());
            for author in &coauthors {
                println!("{author}");
            }
        }
        Err(e) => {
            eprintln!("Error writing to .gitmessage template: {e}");
            process::exit(1);
        }
    });
}

fn select_coauthors(coauthor_initials: &[String]) -> Vec<Author> {
    let all_coauthors = get_available_coauthors();
    let mut coauthors: Vec<Author> = Vec::new();

    for initial in coauthor_initials {
        match all_coauthors.get(initial) {
            Some(coauthor) => coauthors.push(coauthor.clone()),
            None => {
                eprintln!("Error: author with initials {initial} not found");
                process::exit(1);
            }
        }
    }

    coauthors
}
