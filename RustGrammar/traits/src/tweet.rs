pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool,
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    fn summarize_author_info(&self) -> String {
        return format!("(Read More from {}...)", self.summarize_author());
    }
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}, by {}", self.username, self.content);
    }
    fn summarize_author(&self) -> String {
        return format!("@{}", self.username);
    }
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {}", self.headline, self.author);
    }
    fn summarize_author(&self) -> String {
        return format!("@{}", self.author);
    }
}