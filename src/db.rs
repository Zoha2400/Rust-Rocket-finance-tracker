pub fn register_user(name: &str, email: &str) -> bool{
    if name != "" && email != "" {
        println!("Registered {} with email {}", name, email);
        true
    }else {
        println!("Invalid name or email provided.");
        false
    }
}