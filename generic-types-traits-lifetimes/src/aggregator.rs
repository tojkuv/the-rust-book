pub trait Summary {
    fn summarize_author(self: &Self) -> String;
    
    fn summarize(self: &Self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(self: &Self) -> String {
        format!("(Read more from {}...)", self.author)
    }

    fn summarize(self: &Self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(self: &Self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(self: &Self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}