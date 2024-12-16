use std::fmt;

pub struct Album {
    pub title: String,
    pub artist: String,
}
impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Title : {} (Artist: {})", self.title, self.artist)
    }
}


pub struct Albums(pub Vec<Album>);
impl fmt::Display for Albums {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, album|{
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}