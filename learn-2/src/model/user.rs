use std::{thread, time::Duration};

trait Summary {
    fn summarize(&self) -> String {
        String::from("read more...")
    }
}
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{},by {}({})", self.headline, self.author, self.location)
    // }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    caculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(caculation: T) -> Cacher<T> {
        Cacher {
            caculation: caculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(x) => x,
            None => {
                let v = (self.caculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn notify(item: impl Summary) {
    println!("break news:{}", item.summarize());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut larger = list[0];
    for item in list.iter() {
        if *item > larger {
            larger = *item
        }
    }
    larger
}

pub fn test() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    println!("can't use x here:{:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    // let result = longest(string1.as_str(), string2);
    // println!("longet str is:{}", result);
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
