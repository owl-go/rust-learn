use object::{Botton, Icon, Person, Post, Screen};
fn main() {
    let person = Person::new();
    let name = person.get_name();
    println!("{}", name);

    let screen = Screen {
        commponents: vec![
            Box::new(Botton {
                width: 50,
                height: 40,
                lable: String::from("botton"),
            }),
            Box::new(Icon {
                lable: String::from("icon lable"),
                icon: String::from("icon"),
            }),
        ],
    };
    screen.run();

    let mut post = Post::new();
    post.add_text("add first line");
    let post = post.request_review();
    let post = post.approve();
    println!("post content:{}", post.content());
}
