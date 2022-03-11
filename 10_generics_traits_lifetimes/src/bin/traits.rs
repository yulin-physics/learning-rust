use aggregator::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline:String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content:String::from(" The piuttsburgh Penguins once agin are the best \
        hokey team in the NHL.")
    };
    println!("New article available! {}", article.summarize());
}
