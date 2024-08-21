pub fn register_user(name: &str, email: &str) -> bool{
    if name != "" && email != "" {
        println!("Registered {} with email {}", name, email);
        true
    }else {
        println!("Invalid name or email provided.");
        false
    }
}

pub fn get_user(id: u64) -> bool {
    println!("Retrieved user with ID {}", id);
    true
}