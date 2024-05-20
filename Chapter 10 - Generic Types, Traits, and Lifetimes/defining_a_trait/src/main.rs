mod aggregator;

use std::fmt::{Debug, Display};
use crate::aggregator::Summary;
use crate::aggregator::tweet::Tweet;
use crate::aggregator::news_article::NewsArticle;

// TRAIT AS PARAMETERs
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax
pub fn notify_generic<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn _some_function<T, U>(t: &T, u: &U) -> i32
where 
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("{}",t);
    let _u2 = u.clone(); 
    32
}   

// returning Types that implement traits
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

}