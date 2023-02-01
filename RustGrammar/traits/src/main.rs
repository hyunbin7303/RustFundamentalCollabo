use std::fmt::Display;


pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize2(&self) -> String{
        String::from("(Read more)")
    }
    fn summarize_author(&self)-> String;
    fn summarize_author_info(&self)->String
    {
        return format!("(Read More from {}...)", self.summarize_author());
    }
}
// Defining a Details trait by defining the functionality it should include
pub trait Details {
    fn description(&self) -> String;
    fn years_since_release(&self) -> u32;
}
pub struct NewsArticle{
    pub author: String,
    pub headline: String,
    pub content: String
}
pub struct Tweet{
    pub username:String,
    pub content: String,
    pub retweet: bool, 
    pub reply: bool
}
pub struct Movie {
    title: String,
    director: String,
    release_year: u32, 
    genre: String
}

mod Animal {
    pub struct Dog {
        pub name: String,
        pub age: u32, 
        pub owner: String
    }
    impl ToString for Dog {
        fn to_string(&self) -> String {
          return format!("{} is a {} year old dog who belongs to {}.", self.name, self.age, self.owner);
        }
    }
}

impl Summary for NewsArticle {
    fn summarize(&self)-> String{
        return format!("{}, by {}", self.headline, self.author);
    }
    fn summarize_author(&self) -> String{
        return format!("@{}", self.author);
    }
}
impl Summary for Tweet {
    fn summarize(&self) -> String{
        return format!("{}, by {}", self.username, self.content);
    }
    fn summarize_author(&self) -> String{
        return format!("@{}", self.username);
    }
}
  // Implementing an in-built trait ToString on the Dog struct
// Implementing the Details trait on Movie struct
impl Details for Movie{

  // Method returns an overview of the movie
  fn description(&self) -> String{
    return format!("{}, released in {}, is a {} movie directed by {}.", self.title, self.release_year, self.genre, self.director);
  }

  // Method returns the number of years between the writing year of this shot i.e.
  // 2020 and the release year of the movie
  fn years_since_release(&self) -> u32{
    return 2020 - self.release_year;
  }
}
pub fn notify(item:&impl Summary){
    println!("Breaking news ! {}", item.summarize());
}
pub fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news ! {}", item.summarize());
}
fn returns_summarize() -> impl Summary { 
    Tweet {
        username : String::from("Tweet Name"),
        content : String::from("Tweet Content"),
        retweet: true,
        reply : true,
    }
}



struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self{
        Self {x,y}
    }
} 
impl<T: Display + PartialOrd> Pair<T> {
    
}
fn main() {
  let movie1 = Movie{
      title: "Titanic".to_string(),
      director: "James Cameron".to_string(),
      release_year: 1997,
      genre: "historical".to_string()
  };
  println!("{}", movie1.description());
  println!("The movie was released {} years ago.", movie1.years_since_release());
  
  let movie2 = Movie{
      title: "The Dark Knight".to_string(),
      director: "Christopher Nolan".to_string(),
      release_year: 2008,
      genre: "action".to_string()
  };
  println!("\n{}", movie2.description());
  println!("The movie was released {} years ago.", movie2.years_since_release());

  let tweet = Tweet {
      username : String::from("KevinPark7303"),
      content : String::from("Content Checking"),
      reply : false,
      retweet: false,
  };


  let articles = NewsArticle {

      author: String::from("Kevin Park"),
      headline: String::from("Head Line"),
      content: String::from("Content of the head line"),
  };
  let dog = Animal::Dog{name: "Frodo".to_string(), age: 3, owner: "Maryam".to_string()};
  println!("{}", dog.to_string());


  println!("Tweet Summary : {}", tweet.summarize());
  println!("Tweet username : {}", tweet.summarize_author_info());
  println!("News Article Summary : {}", articles.summarize());
  notify(&articles);

  println!("Return Tweet : {}", returns_summarize().summarize());

}