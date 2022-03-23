
#[derive(Debug)]
pub struct Word {
    pub letter : String,
    pub meaning: String,
    pub synonym: String,
}
impl Word {
    pub fn new(letter: &str, meaning: &str, synonym: &str)-> Self{
        Self {
            letter : letter.to_string(),
            meaning : meaning.to_string(),
            synonym: synonym.to_string()
        }
    }
}