use clap_mangen::Man;
use std::env;
use std::path::Path;

#[path = "src/cli.rs"]
mod cli;

macro_rules! generate_manpage {
    ($struct:ident) => {
        let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or("target".to_string());
        let output_dir = Path::new(&target_dir).join(env::var("PROFILE").unwrap());

        let cmd = cli::$struct::command();
        let cmd_name = format!("{}.1", cmd.get_name());
        let man = Man::new(cmd);
        let mut buffer: Vec<u8> = Default::default();
        man.render(&mut buffer)?;
        std::fs::write(output_dir.join(cmd_name), buffer)?;
    };
}

fn main() -> std::io::Result<()> {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or("target".to_string());
    let output_dir = Path::new(&target_dir).join(env::var("PROFILE").unwrap());

    let cmd = cli::git_mob_cmd();
    let man = Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(output_dir.join("git-mob.1"), buffer)?;

    for subcommand in cmd.get_subcommands() {
        let man = Man::new(subcommand.clone());
        let mut buffer: Vec<u8> = Default::default();
        man.render(&mut buffer)?;
        std::fs::write(output_dir.join(format!("git-mob-{}.1", subcommand.get_name())), buffer)?;
    }

    Ok(())
}
