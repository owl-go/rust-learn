use chrono::prelude::*;
fn main() {
    let local = Local::now();

    println!("local timestamp:{}", local.timestamp());
    println!("Hello, world!");
    println!("{}", local.to_string());
    let utc = Utc::now();
    println!("timeStamp:{}", utc.timestamp()); //获取时间戳

    println!(
        "year:{},month:{},day:{},hour:{},minute:{},second:{}",
        local.year(),
        local.month(),
        local.day(),
        local.hour(),
        local.minute(),
        local.second()
    ); //年月日
    let time_str = local.format("%Y-%m-%d %H:%M:%S").to_string(); //时间格式化
    println!("time str:{}", time_str);
    let now = Local.timestamp(1634545287, 0); //将时间戳转化为当前时间
    println!(
        "now hour:{},minute:{},second:{}",
        now.hour(),
        now.minute(),
        now.second()
    );
    let week = now.weekday();
    println!("current week:{},day0:{}", week, now.day0());
    let today = Local::today();
    println!("today:{}", today);
    assert_eq!(Local.ymd(2021, 10, 18).weekday(), Weekday::Mon);
    if let chrono::LocalResult::None = Local.ymd_opt(2021, 10, 32) {
        println!("No such local time");
    } else {
        // println!("str:{}", str);
    }
}
