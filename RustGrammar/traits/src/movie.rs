use core::fmt;


pub struct Movie {
    pub title: String,
    pub director: String,
    pub release_year: u32,
    pub genre: String,
}
pub trait Details {
    fn description(&self) -> String;
    fn years_since_release(&self) -> u32;
}
impl Details for Movie {
    fn description(&self) -> String {
        return format!(
            "{}, released in {}, is a {} movie directed by {}.",
            self.title, self.release_year, self.genre, self.director
        );
    }
    // Method returns the number of years between the writing year of this shot.
    // 2020 and the release year of the movie
    fn years_since_release(&self) -> u32 {
        return 2020 - self.release_year;
    }
}

impl fmt::Display for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Movie Title:{}, Director:{}", self.title, self.director)
    }
}