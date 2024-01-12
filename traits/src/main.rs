pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

//imple {trait} for {struct}
impl Summary for NewsArticle {
    // commented out to show default function
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// overrides the default implementation
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

// traits can be used to define a shared behavior (methods) accross different structs
pub trait Summary {
    // only specify the signature but no method body
    // for every type that implements this trait must have this method
    // fn summarize(&self) -> String; 

    // default implementation
    fn summarize(&self) -> String {
        String::from("Default summarization...")
    }

    fn summarize_author(&self) -> String;
}

// returns something that implements Summary trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, are you probably already know, people."
        ),
        reply: false,
        retweet: false,
    }
}

// item can be any type that implements summary trait

// pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary)
// this makes it so that item1 has to have the Summary and Display trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// the code above is syntactic sugar for this
// the impl syntax does this
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

/* 
pub fn some_function<T, U>(t: &T, u:&U) -> i32 
    where T: Display + Clone,
          U: Clone + Debug,
{
    //..
}
*/

fn main() {
    let tweet = Tweet {
        username: String::from("@patrick"),
        content: String::from("Tweet Content"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("Patrick Mediodia"),
        headline: String::from("Headline"),
        content: String::from("Content"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);

    println!("{}", returns_summarizable().summarize());
}