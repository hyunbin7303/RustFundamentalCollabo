
#[derive(Debug)]
pub struct Word {
    pub letter : String,
    pub meaning: String,
    pub synonyms: Vec<String>,
}
impl Word {
    pub fn new(letter: &str, meaning: &str, synonyms: Vec<String>)-> Self{
        Self {
            letter : letter.to_string(),
            meaning : meaning.to_string(),
            synonyms: synonyms
        }
    }
}