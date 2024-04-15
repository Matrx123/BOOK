//Important thing is that, a user will have to bring the trait as well as types into scope.
//here Summary is Trait and NewsArticle is struct which implements Summary trait.

use traits_ex::{NewsArticle, Summary, Tweet};

mod traits_ex;
fn main() {
  let article = NewsArticle {
      author: String::from("Vipin Joshi"),
      headline: String::from("Next JS development"),
      content : String::from("Learn how to develop in Next js using concepts of Reactjs and Typescript.")
  };
  let tweet = Tweet {
      username: String::from("@pin_v_pin"),
      retweet: true,
      reply: true,
      content: String::from("A new video about Nextjs is uploaded on Youtube.")
  };
  println!("Summary : {:?}", article.summarize());
  println!("Tweet Summary : {:?}", tweet.summarize())
}
