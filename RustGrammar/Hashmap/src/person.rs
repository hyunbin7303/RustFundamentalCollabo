pub struct User<'a> {
    pub userid: u32,
    pub username: &'a str,
}

pub struct Person {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}


impl Person {
    fn new(username: String, email: String) -> Person {
        Person {
            username: username,
            email : email,
            active : true,
            sign_in_count : 0,
        }
    }
}

//https://blog.logrocket.com/how-to-read-files-rust/ Todo List 
//https://www.reddit.com/r/rust/comments/5t5zq1/when_to_use_str_over_string_in_a_struct/
//https://stackoverflow.com/questions/38761048/how-to-pass-a-hashmap-to-a-function-in-rust
//https://www.ralfj.de/projects/rust-101/part04.html
