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
struct Albums(pub Vec<Album>);
impl fmt::Display for Albums {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, album|{
            result.and_then(|_| writeln!(f, "{}", album))
        }) 
    }
}
struct IpAddress(u8, u8, u8, u8);
impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

struct Password(String);
impl fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        write!(f, "***********")
    }
}


#[derive(Debug)]
struct Post {
    id : i32,
    tags : Vec<String> 
}
// impl fmt::Display for Post {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(f, "Tags found.");

//         for tag in self.tags.iter() {
//             writeln!(f, "- {}", tag)?;
//         }
//         write!(f, "Done.");
//         Ok(())
//     }
// }


fn main() {

    let first_album = Album {
        title: "Desperado".into(),
        artist: "The Beatles".into(),
    };
    println!("{}", first_album);
    println!("Creating Own Albums struct in order to display Album in vec.");
    let albums = Albums(vec![
        first_album,
        Album {
            title: "Dark Side of the Moon".into(),
            artist: "Kevin Park".into(),
        },
    ]);

    println!("{}", albums);

    let ip_address = IpAddress(152, 40, 50, 20);
    println!("{}", ip_address);

    let posts = vec! [
        Post { id : 1, tags : vec![ "Test".into(), "algorithm".into()] },
        Post { id : 2, tags : vec![ "programming".into(), "dotnet".into()] },
        Post { id : 3, tags : vec![ "programming".into(), "Rust".into()] },
    ];
    println!("{:?}", posts);


    let userpwd: String = "Password#1234".to_string();
    let secure_pwd: Password = Password(userpwd.clone());
    println!("Safe pwd : {}", secure_pwd);
}