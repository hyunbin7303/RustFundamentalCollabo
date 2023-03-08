use std::fmt;
#[derive(Debug)]
struct Album {
    pub title: String,
    pub artist: String,
}

fn main() {
    let albums = vec![
        Album {
            title: "Desperado".into(),
            artist: "The Beatles".into(),
        },
        Album {
            title: "Dark Side of the Moon".into(),
            artist: "Kevin Park".into(),
        },
    ];
    println!("Displaying testing using the fm:: Display - {:?}", albums);

}
impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.artist)
    }
}
