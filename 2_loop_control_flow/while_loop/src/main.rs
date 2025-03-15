use std::io;

fn main() {
    let mut input = String::new();
    while input.trim() != "q" {
        input.clear();
        println!("Enter a word or 'q' to quit");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("You entered: {}", input.trim());
    }
    println!("Exiting program...");
}