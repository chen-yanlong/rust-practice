// traits allows us to define methods accross different structs

pub struct NewsArticle {
    pub author: String, 
    pub headline: String, 
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
pub struct Tweet {
    pub username: String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool,
}    

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.content, self.username)
    }
}

pub trait Summary {
    fn summarize(&self) -> String{
        // define default impl here
        String::from("Read more...")
    }
}

// item can be anything that implements summary
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news: {}", item.summarize())
// }

// another form
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    //...
}
// another form
// pub fn notify<T>(item1: &T, item2: &T) -> i32 
//     where T: Summary + Debug
// {
// }



fn main() {
    println!("Hello, world!");
}
