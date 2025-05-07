//This trait tells us that a struct can implement a summarize function which will return a String
//Similar to Java interfaces
pub trait Summary {

    //&self is telling us that will be called from an istance through instance.summarize
    //No implementation must be now explicited (just like interfaces), but can have a default one
    fn summarize(&self) -> String {
        String::from("(Read more...)") // returning the string, note no semicolon
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

//Here we implement the Summary Trait for NewsArticle
impl Summary for NewsArticle {

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//Here we implement the Summary Trait for Tweet
impl Summary for Tweet {

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}