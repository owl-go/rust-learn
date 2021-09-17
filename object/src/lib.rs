pub struct Person {
    name: String,
    age: u32,
    gender: u8,
}
impl Person {
    pub fn new() -> Person {
        Person {
            name: String::from(""),
            age: 0,
            gender: 1,
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_age(&self) -> u32 {
        self.age
    }
    pub fn get_gender(&self) -> u8 {
        self.gender
    }
}

pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub commponents: Vec<Box<dyn Draw>>, //Box里面的元素都实现了Draw这个trait
}
impl Screen {
    pub fn run(&self) {
        for commponent in self.commponents.iter() {
            commponent.draw();
        }
    }
}

pub struct Botton {
    pub width: u32,
    pub height: u32,
    pub lable: String,
}

impl Draw for Botton {
    fn draw(&self) {
        println!(
            "Botton: width={},height={},lable={}",
            self.width, self.height, self.lable
        )
    }
}

pub struct Icon {
    pub lable: String,
    pub icon: String,
}
impl Draw for Icon {
    fn draw(&self) {
        println!("Icon: lable={},icon={}", self.lable, self.icon)
    }
}
//定义一个博文状态改变
// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }
// pub trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
// }
//正式发布的post
pub struct Post {
    content: String,
}
//草稿post
pub struct DraftPost {
    content: String,
}
//待审批的post
pub struct PendingReviewPost {
    content: String,
}
impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    //请求审批 返回待审批的post
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
