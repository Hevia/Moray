use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

struct Variable {
    identifer: String,
    isMut: bool
}


fn main() -> Result<(), Error> {
    let path = "D:/Desktop/code/moray/moray-lang/tests/assign.my";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        for token in line.unwrap().split_whitespace() {
            println!("{}", token);
        }
    }

    Ok(())
}
