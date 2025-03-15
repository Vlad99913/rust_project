use std::io;

fn sum (numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for n in numbers {
        sum += n;
    }
    sum
}

fn main() {
    println!("Enter a list of numbers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i32> = input.split_whitespace().map(|s| s.parse().expect("Invalid input")).collect();
    let sum = sum(&numbers);
    println!("Sum: {}", sum);
}
