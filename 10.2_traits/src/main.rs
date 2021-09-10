use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

trait Awesome {
    fn wow(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}", self)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//Trait as parameter
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//Trait bound
fn warn<T: Summary>(item: &T) {
    println!("Watch out! {}", item.summarize());
}

fn broadcast(item1: &impl Summary, item2: &(impl Summary + Display)) {
    println!("Tell everyone! {}", item1.summarize());
    println!("Tell everyone! {}", item2.summarize());
}

fn whisper<T: Display + Summary, U>(item1: &T, item2: &U) where U: Summary {
    println!("Tell no one! {}", item1.summarize());
    println!("Tell no one! {}", item2.summarize());
}

fn new(username: String, content: String, reply: bool, retweet: bool) -> impl Summary {
    Tweet {
        username,
        content,
        reply,
        retweet
    }
    // By using -> impl Summary we cant return Tweet or NewsArticle depending on the parameters,
    // because compiler will not know what type should be.
}

// Solution of 10.1_generics
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = new(
        String::from("something"), 
        String::from("this is supposed to be something else"), 
        false, 
        false
    );

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Surprise"),
        location: String::from("Away"),
        author: String::from("Unknown"),
        content: String::from("Can't say")
    };
    
    println!("1 new article: {}", article.summarize());

    notify(&tweet);
    warn(&article);
    broadcast(&tweet, &article);
    whisper(&article, &tweet);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);


    let pair = Pair { x: 5, y: 10 };
    pair.cmp_display();
}
