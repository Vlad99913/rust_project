#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn new(first_name: &str, last_name: &str, age: u8) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
        }
    }
    
}

fn main() {
    let  john = Person::new("John", "Doe", 30);
    // print the full name of the person after saying hello and then say your age
    println!("Hello, {}! You are {} years old.", john.get_full_name(), john.age);
}
