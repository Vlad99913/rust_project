use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = args[1].clone();
    let file = File::open(filename);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
        
    }
}
