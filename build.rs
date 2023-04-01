use clap::CommandFactory;
use clap_mangen::Man;
use std::env;
use std::path::Path;

#[path = "src/cli.rs"]
mod cli;

fn main() -> std::io::Result<()> {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or("target".to_string());
    let output_dir = Path::new(&target_dir).join(env::var("PROFILE").unwrap());

    // git-mob docs
    let cmd = cli::GitMob::command();
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(output_dir.join("git-mob.1"), buffer)?;

    // git-solo docs
    let cmd = cli::GitSolo::command();
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(output_dir.join("git-solo.1"), buffer)?;

    // git-add-coauthor docs
    let cmd = cli::GitAddCoauthor::command();
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(output_dir.join("git-add-coauthor.1"), buffer)?;

    // git-edit-coauthor docs
    let cmd = cli::GitEditCoauthor::command();
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(output_dir.join("git-edit-coauthor.1"), buffer)?;

    // git-delete-coauthor docs
    let cmd = cli::GitDeleteCoauthor::command();
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(output_dir.join("git-delete-coauthor.1"), buffer)?;

    Ok(())
}
