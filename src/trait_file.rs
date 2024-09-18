pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
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
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

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

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// OR

// Trait bound
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }



// we can do this as well
// pub fn notify(item1: &impl Summary, item2: &impl Summary) { // for using different types
// }

// OR

// pub fn notify<T: Summary>(item1: &T, item2: &T) { // for enforcing same types
// }



// Specifying Multiple Trait Bounds with the + Syntax

// pub fn notify(item: &(impl Summary + Display)) {

// OR

// pub fn notify<T: Summary + Display>(item: &T) {



// Clearer Trait Bounds with where Clauses

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// TO

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {



pub fn returns_newsarticle_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("Final Parsec Problem"),
        location: String::from("Bengaluru"),
        author: String::from("Sourav"),
        content: String::from(
            "The merger of the two SMBHs stall at the final parsec",
        ),
    }
}