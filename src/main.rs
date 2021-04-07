use clap::{App, Arg, SubCommand};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{ChildStdout, Command, Stdio};

fn main() {
    let matches = App::new("moray")
        .version("0.1")
        .author("Anthony Hevia. <anthony@hevia.dev>")
        .about("Rust's Cargo-style tooling for Python")
        .subcommand(
            SubCommand::with_name("new")
                .about("creates a new project & nested virtual enviroment")
                .arg(Arg::with_name("project_name").index(1).required(true)),
        )
        .subcommand(SubCommand::with_name("install").about("Installs dependencies"))
        .subcommand(SubCommand::with_name("run").about("runs your python program"))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("new") {
        let proj_name = matches.value_of("name").unwrap();

        // TODO: better error handling
        let op_result = create_folder(proj_name).unwrap();
        let _display_result = display_output(op_result);
        let op_result = create_venv(proj_name).unwrap();
        let _display_result = display_output(op_result);
    }
}

fn display_output(stdout: ChildStdout) -> Result<(), Error> {
    let reader = BufReader::new(stdout);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
    Ok(())
}

fn create_folder(folder_name: &str) -> Result<ChildStdout, Error> {
    let stdout = Command::new("cmd")
        .args(&["/C", format!("mkdir {}", folder_name).as_str()])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    Ok(stdout)
}

fn create_venv(folder_name: &str) -> Result<ChildStdout, Error> {
    let stdout = Command::new("cmd")
        .args(&[
            "/C",
            format!("python -m venv ./{}/venv", folder_name).as_str(),
        ])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    Ok(stdout)
}

fn activate_venv(folder_name: &str, further_commands: &str) -> Result<ChildStdout, Error> {
    let stdout = Command::new("cmd")
        .args(&[
            "/C",
            format!(
                ".\\{}\\venv\\Scripts\\activate & {}",
                folder_name, further_commands
            )
            .as_str(),
        ])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    Ok(stdout)
}
