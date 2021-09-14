use std::ops::SubAssign;

fn main() {
    let mut a = 12;
    a = 25;
    println!("a is:{0}", a);

    const CA: i32 = 100; //constant
    println!("const a is {0}", CA);
    //Shadowing
    let x = 2;
    let x = x + 1;
    let x = x * 2;
    println!("The Value of x is:{}", x);
    //DataType
    let sum = 5 + 10; // 加
    let difference = 95.5 - 4.3; // 减
    let product = 4 * 30; // 乘
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余
    println!(
        "sum:{},diffefence:{},product:{},quotient:{},remainder:{}",
        sum, difference, product, quotient, remainder
    );
    //tuple
    let tup: (i32, f64, u8, bool) = (500, 12.2, 10, false);
    let (x, y, z, k) = tup;
    println!("x:{},y:{},z:{},k:{}", x, y, z, k);
    //array
    let arr_a = [1, 2, 3, 4, 5];
    for elem in arr_a {
        println!("{}", elem);
    }
    println!("sum:{}", add(10, 20));
    //condition
    let result = add(50, 40);
    if result > 100 {
        println!("biger than 100");
    } else {
        println!("less than 100");
    }
    //loop
    let mut num = 1;
    while num != 4 {
        println!("{}", num);
        num += 1;
    }
    let a = [10, 20, 30, 40, 50];
    for elem in a.iter() {
        println!("elem:{}", elem);
    }
    for i in 0..a.len() {
        println!("a[{}]:{}", i, a[i]);
    }
    let mut i = 0;
    loop {
        println!("loop:{},{}", i, a[i]);
        if i == 4 {
            break;
        }
        i += 1;
    }
    let s = String::from("hello");
    println!("s addr is:{:p}", s.as_ptr());
    let s1 = take_and_give_back(s);
    println!("s1 addr is:{:p}", s1.as_ptr());
    //
    let give_str = give_owner_ship();
    println!("give_str addr is:{:p}", give_str.as_ptr());

    let clone_str = give_str.clone();

    println!("clone_str addr is:{:p}", clone_str.as_ptr());

    //slice
    let s = String::from("broadcast");
    let s1 = &s[0..5];
    let s2 = &s[5..9];
    println!("{}={}+{}", s, s1, s2);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn take_and_give_back(a_string: String) -> String {
    println!("addr:{:p}", a_string.as_ptr());
    a_string
}
fn give_owner_ship() -> String {
    let s = String::from("hello");
    println!("give_owner_ship addr:{:p}", s.as_ptr());
    s
}
