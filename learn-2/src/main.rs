mod model;
use model::user;
mod common;
use common::uks;
// use model::get_test;

#[derive(Debug)]
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32,
}
impl Site {
    fn get_domain(&self) -> &str {
        &self.domain[..]
    }
}
#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
enum Book {
    Paper(u32),
    Electronic(String),
}

fn main() {
    let site = Site {
        domain: String::from("senggo.com"),
        name: String::from("senggo"),
        nation: String::from("china"),
        found: 1,
    };
    println!("site:{:?}", site);
    println!("domain:{}", site.get_domain());

    let red = Color(255, 200, 180);
    println!("color:{:?}", red);

    let book = Book::Paper(1001);
    match book {
        Book::Paper(i) => {
            println!("{}", i);
        }
        Book::Electronic(str) => {
            println!("{}", str);
        } // _ => {
          //     println!("other options");
          // }
    }
    let ebook = Book::Electronic(String::from("url"));
    if let Book::Electronic(url) = ebook {
        println!("electronic book:{}", url);
    } else {
        println!("not electronic book");
    }
    user::get_test();
    let total = uks::add(10, 90);
    println!("total:{}", total);
    common::test();
}
