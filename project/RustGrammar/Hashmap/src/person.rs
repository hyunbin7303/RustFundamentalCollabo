struct Person {
    active: bool,
    username: Stri,ng,
    email: String
    sign_in_count: u64,
}


impl Person {
    fn new(username: &str, email: &str) -> Person {
        Person {
            username: username.to_string(),
            email : email.to_string(),
            active : true,
            sign_in_count : 0
        }
    }
}