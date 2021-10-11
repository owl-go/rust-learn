use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, ErrorKind, Read},
    net::IpAddr,
};

pub fn get_test() {
    println!("hello test");
}
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub fn test() {
    let p1 = Point { x: 5, y: 4 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x={},p3.y={}", p3.x, p3.y);
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
