use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{ChildStdout, Command, Stdio};
use clap::{Arg, App, SubCommand};

fn main() {
     let matches = App::new("moray")
                          .version("0.1")
                          .author("Anthony Hevia. <anthony@hevia.dev>")
                          .about("Cargo-style Python tooling")
                          .subcommand(SubCommand::with_name("new")
                                      .about("creates a new project & virtual enviroment")
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
                          .subcommand(SubCommand::with_name("install")
                                      .about("Installs dependencies")
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
                          .subcommand(SubCommand::with_name("run")
                                      .about("runs your python program")
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
                          .get_matches();

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

fn create_venv(venv_name: &str) -> Result<ChildStdout, Error> {
    let stdout = Command::new("cmd")
        .args(&["/C", format!("python -m venv ./{}", venv_name).as_str()])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    Ok(stdout)
}

fn activate_venv(venv_name: &str, further_commands: &str) -> Result<ChildStdout, Error> {
    let stdout = Command::new("cmd")
        .args(&[
            "/C",
            format!(".\\{}\\Scripts\\activate & {}", venv_name, further_commands).as_str(),
        ])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    Ok(stdout)
}