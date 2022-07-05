use trait_study::{returns_summarizable, test_study, NewArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("herman"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    println!("notify {}", test_study::notify(&article));

    let article = &tweet;
    println!("notify {}", test_study::notify2(&tweet, &article));

    let t = returns_summarizable();
    t.summmarize_author();
}
