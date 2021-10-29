static mut COUNTER: u32 = 0;
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("{}", *r1);
        println!("{}", *r2);
        *r2 = 10;
        println!("{}", *r1);
        println!("{}", *r2);
        println!("{}", num);
    }
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // let mut r = v;
    let (a, b) = r.split_at_mut(3);
    println!("{:?}-{:?}", a, b);

    unsafe {
        //修改静态变量是不安全的操作
        COUNTER += 1;
        println!("counter:{}", COUNTER);
    }
}
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just call function from c");
}
