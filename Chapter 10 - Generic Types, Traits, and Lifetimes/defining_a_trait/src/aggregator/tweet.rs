use crate::aggregator::Summary;

pub struct Tweet {
    pub username: String,
    pub content: String,
    _reply: bool,
    _retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Tweet {
    pub fn new() -> Self {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            _reply: false,
            _retweet: false,
        }
    }
}