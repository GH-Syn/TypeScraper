#!ignore(unused_variables)
use colored::*;

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub replies: Vec<String>,
    pub retweets: u128,
    pub likes: u128,
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn feedback(&self) -> String;
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn feedback(&self) -> String {
        format!("{} 👍, {} 🔁\nSee {} replies",
        self.likes.to_string().yellow().bold(),
        self.retweets.to_string().blue().bold(),
        self.replies.len())
    }
}

fn print_tweets(tweets: &Vec<Tweet>) {
    /* Prints tweets with all of it's summarized properties */
    for tweet in tweets {
        println!("{}", tweet.summarize());
        println!("{}", tweet.feedback());
    }
}

fn main() {
    // Array of tweets where tweets are stored
    let _tweets:Vec<Tweet> = Vec::new();
    // Create an instance of a tweet
    let tweet = Tweet {
        username: String::from("Joshua Rose"),
        content: String::from("Donkeys are pretty cool!"),
        replies: Vec::new(),
        retweets: 3,
        likes: 39
    };

    // Add tweet to tweets
    let tweets:Vec<Tweet> = Vec::from([tweet]);
    print_tweets(&tweets)
}
