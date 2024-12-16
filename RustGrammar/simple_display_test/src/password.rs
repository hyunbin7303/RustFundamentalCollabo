use std::fmt;

pub struct Password(pub String);
impl fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "***********")
    }
}
