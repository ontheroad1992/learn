pub trait Summary {
    fn summmarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    // 存在默认就不会强制要求，编写此代码
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summmarize_author(&self) -> String {
        format!("@{}", self.author)
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
        format!("{}: {}", self.username, self.content)
    }

    fn summmarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub mod test_study {
    use std::fmt::Debug;

    use crate::Summary;

    pub fn notify<T: Summary>(item: &T) -> String {
        let s = item.summarize();
        println!("Breaking news! {}", s);
        s
    }

    pub fn notify2<T: Summary>(item1: &T, item2: &T) -> String {
        format!("{}, {}", item1.summarize(), item2.summarize())
    }

    // pub fn notify3(item: &(impl Summary + Display)) {}
    // pub fn notify3<T: Summary + Display>(item: &T) {}
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {
    // }
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
