use traits_demo::{NewsArticle, Summary, notify,notify_tb};

mod traits_demo;

fn main() {
    let article = NewsArticle {
        author: String::from("vipin joshi"),
        heading: String::from("Hello world"),
        content: String::from("This is a traits demo")
    };
    println!("summary of article1 : {:?}", article.summarize());
    println!("default summary : {:?}", article.def_summary());
    println!("{:?}", article.def_summary_calling_summarize());
    println!("{:?}", notify(&article));
    println!("{:?}", notify_tb(&article));
}
