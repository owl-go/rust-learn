mod libs;

use crate::libs::{notify, longest};
use libs::Summary;
use libs::Tweet;
use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    for elem in v.iter() {
        println!("element:{}", elem);
    }
    match v.get(100) {
        Some(third) => {
            println!("{}", third);
        }
        None => {
            println!("none");
        }
    }
    println!("item:{}", v[2]);
    for elem in &mut v {
        *elem += 5;
        println!("{}", elem);
    }
    let s = "this is a demo str";
    let s1 = s.to_string();
    let s2 = String::from("中文文字");
    println!("{}-{}", s1, s2);

    let mut s3 = String::from("foo");
    s3.push_str("bar");
    println!("s3:{}", s3);

    let s4 = s1 + &s2;
    println!("s4:{}", s4);

    let s5 = format!("{}-{}-{}", s2, s3, s4);
    println!("{}", s5);

    //HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("lilei"), 98);
    let teams = vec![String::from("blue"), String::from("yellow")];
    let inital_scores = vec![90, 23];
    let team_score: HashMap<_, _> = teams.iter().zip(inital_scores.iter()).collect();

    let lilei = scores.get("lilei");
    match lilei {
        Some(v) => {
            println!("{}", v);
        }
        None => {
            println!("none");
        }
    }
    let text = "hello world wanderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);

    let tweet = Tweet {
        username: String::from("hourse_ebook"),
        content: String::from("of course any body can tweet any messages"),
        reply: false,
        retwee: false,
    };
    println!("1 new tweet:{} ", tweet.summarize());
    notify(tweet);
    let longest=longest("i am from china","i am from france");
    println!("{}",longest);
}
