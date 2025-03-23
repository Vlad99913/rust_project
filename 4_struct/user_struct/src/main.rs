#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn activate(&mut self) {
        self.active = true;
    }

    fn from_email(email: String) -> User {
        let username = email.split("@").next().unwrap().to_string();
        User {
            username,
            email,
            active: true,
        }
    }
    
}

fn main() {
    let email = "test@example.com".to_string();
    let mut user = User::from_email(email);
    println!("User: {}", user.username);
    println!("Email: {}", user.email);
    println!("Status: {}", user.active);
    user.deactivate();
    println!("Status: {}", user.active);
}
