mod Animal {
    pub struct Dog {
        pub name: String,
        pub age: u32,
        pub owner: String,
    }
    impl ToString for Dog {
        fn to_string(&self) -> String {
            return format!(
                "{} is a {} year old dog who belongs to {}.",
                self.name, self.age, self.owner
            );
        }
    }
}
