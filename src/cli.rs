use clap::{Arg, arg};

pub fn git_mob_cmd() -> clap::Command {
clap::Command::new("git-mob")
        .bin_name("git-mob")
        .author("Martin Frost, martin@frost.codes")
        .version("0.0.0")
        .about("A command-line tool for social coding")
        .subcommand(
            clap::Command::new("add")
                .alias("add-coauthor")
                .about("Add a coauthor to the database")
                .arg(arg!(<handle> "The coauthor's handle"))
                .arg(arg!(<name> "The coauthor's name, in quotes"))
                .arg(arg!(<email> "The coauthor's email"))
        )
        .subcommand(
            clap::Command::new("edit")
                .alias("edit-coauthor")
                .about("Edit a coauthor in the database")
                .arg(arg!(<handle> "The coauthor's handle"))
                .arg(arg!(<name> "The coauthor's name, in quotes"))
                .arg(arg!(<email> "The coauthor's email"))
        )
        .subcommand(
            clap::Command::new("remove")
                .alias("remove-coauthor")
                .alias("rm")
                .about("Remove a coauthor from the database")
                .arg(arg!(<handle> "The coauthor's handle"))
        )
        .subcommand(
            clap::Command::new("solo")
            .about("Disband the mob. Continue coding on your own")
            .arg_required_else_help(false))
        .subcommand(
            clap::Command::new("list")
            .about("List available coauthors in the database")
            .arg_required_else_help(false))
        .subcommand(
            clap::Command::new("with")
                .about("Choose coauthors to join your mob")
                .arg_required_else_help(true)
                .arg(Arg::new("override").short('o').long("override").help("Override default author").value_name("handle"))
                .arg(arg!([handles] "The coauthors you want in your mob").num_args(1..)))
}
