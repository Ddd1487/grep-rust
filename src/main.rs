use std::{fs::File, io::BufReader};
use std::io::BufRead;
use clap::Parser;

#[derive(Parser)]
struct Cli {
        pattern: String,
        path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    //println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let file = File::open(&args.path).expect("File couldn't be opened");
    let content = BufReader::new(file);
    for line in content.lines() {
        let line = line.expect("could not read line");
        if line.contains(&args.pattern) {
            println!("{}",line);
        }
    }
}
