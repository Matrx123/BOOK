// Traits also support default implementation, so that we are not forced to implement :P
pub trait Summary {
    //required implementation, which must be implemented on the type which is using this Trait.
    fn summarize(&self) ->String ;
    //default implementation
    fn def_summary(&self) -> String {
        String::from("Default implementation...")
    }
    //Note: Default implementation can call other methods on the same traits as well.
    // however, vice-versa is not possible. "ofcourse"
    //see below :)
    fn def_summary_calling_summarize(&self) -> String {
        format!("calling non-default method from default method = {}", self.summarize())
    }
}

pub struct NewsArticle {
    pub author: String,
    pub heading: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("Summary :: {} by {} - {}",&self.heading, &self.author, &self.content)
    }
}

//Traits can also be used as parameters, and we can call any method on that trait.
// this parameter accepts any type that implementes the specifies trait (i.e Summary)
// we can't pass String or i32 as they don't implement Summary Trait.
//see below
pub fn notify(item: &impl Summary) -> String {
    format!("Breaking news ... {}", item.summarize())
}

//Trait Bound Syntax
pub fn notify_tb<T: Summary>(item : &T) -> String {
    format!("Trait bound syntax {}", item.summarize())
}

//----------//
// **Note**:
//----------//
//-----------------------------------------------------------------------------------//
// 1-Coherence rule:
// :: This is also known as orpahan rule, 
//    which states,
//      to implement either a foreign trait to our type or our trait to a foreign type,
//      one of our traits or type must be local to our crate. 
//    we can't implement foreign trait to foreign type.
// Rules:: //
//       1- The trait is defined in your crate and the type is foreign.
//       2- The type is defined in your crate and the trait is foreign.
//       3- Both the type and the trait are defined in your crate.