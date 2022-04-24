use std::fmt;
#[derive(PartialEq, Debug)]
struct Album {
    pub title: String,
    pub artist: String,
}
struct User {
    name: String,
    email: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{} <{}>", self.name, self.email)
    }
}

impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.artist)
    }
}

// impl fmt::Display for Vec<Album> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.iter().fold(Ok(()), |result, album| {
//             result.and_then(writeln!(f, "{:?}", album))
//         })
//     }
// }




fn main() {
    let albums = vec![
        Album {
            title: "Sgt. Pepper's Lonely Hearts Club Band".into(),
            artist: "The Beatles".into(),
        },
        Album {
            title: "Dark Side of the Moon".into(),
            artist: "Pink Floyd".into(),
        },
    ];
    let new_user = User {
        name: "Benjamin Lannon".to_string(),
        email: "email@example.com".to_string()
    };
    // println!("{}", albums);
    println!("{}", new_user); // Pri

}