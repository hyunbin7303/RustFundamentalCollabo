//Source Ref from :
//https://blog.logrocket.com/rust-traits-a-deep-dive/
use traits::{movie::{Details, Movie}, tweet::{NewsArticle, Summary, Tweet}};
use std::{path::Display, string::ToString};

trait ContainerAnnotation<'a> {
    type ItemIterator: Iterator<Item = &'a u8>;
    fn items(&'a self) -> Self::ItemIterator;
}
// Defining a Details trait by defining the functionality it should include
pub fn notify(item: &impl Summary) {
    println!("Breaking news ! {}", item.summarize());
}
pub fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news ! {}", item.summarize());
}

// pub fn notify_generic<T: Summary>(item1: &T, item2: &T) {
//     println!("Breaking news ! {}", item.summarize());
// }
pub fn returns_summarize() -> impl Summary {
    Tweet {
        username: String::from("Tweet Name"),
        content: String::from("Tweet Content"),
        retweet: true,
        reply: true,
    }
}


// TODO - check out how to use this one properly.
pub trait AllTrait {
    fn use_self(self);
    fn use_immutable(&self);
    fn use_mutable(&mut self);
    // fn use_self_for(self: Self);
    // fn use_immutable(self: &Self);
    // fn use_mutable(self: &mut Self);

    // type AssociatedType;
    // fn check_associated(arg: Self::AssociatedType);
}


fn main() {
    let num_string = 5.to_string();
    let five = ToString::to_string(&5); // called by namespaced by the trait.

    let movie1 = Movie {
        title: "Titanic".to_string(),
        director: "James Cameron".to_string(),
        release_year: 1997,
        genre: "historical".to_string(),
    };
    println!("{}", movie1); // Using Display Trait.
    println!("{}", movie1.description());
    println!(
        "The movie was released {} years ago.",
        movie1.years_since_release()
    );

    let movie2 = Movie {
        title: "The Dark Knight".to_string(),
        director: "Christopher Nolan".to_string(),
        release_year: 2008,
        genre: "action".to_string(),
    };
    println!("\n{}", movie2.description());
    println!(
        "The movie was released {} years ago.",
        movie2.years_since_release()
    );

    let tweet = Tweet {
        username: String::from("KevinPark7303"),
        content: String::from("Content Checking"),
        reply: false,
        retweet: false,
    };

    let articles = NewsArticle {
        author: String::from("Kevin Park"),
        headline: String::from("Head Line"),
        content: String::from("Content of the head line"),
    };
    println!("Tweet Summary : {}", tweet.summarize());
    println!("Tweet username : {}", tweet.summarize_author_info());
    println!("News Article Summary : {}", articles.summarize());
    notify(&articles);

    println!("Return Tweet : {}", returns_summarize().summarize());
}
