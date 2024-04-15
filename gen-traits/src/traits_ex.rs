pub struct NewsArticle {
    pub author : String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username : String,
    pub retweet: bool,
    pub reply: bool,
    pub content: String,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize (&self) -> String {
        format!("{:?} by {:?}",self.headline ,self.author)
    }
}

impl Summary for Tweet {
    fn summarize (&self) -> String {
        format!("A Tweet by {:?}, regarding {:?}", self.username, self.content)
    }
}