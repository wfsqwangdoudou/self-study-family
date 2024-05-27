use rand::Rng;
use std::io;

fn main() {
    println!("猜猜正整数！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("秘密号码是：{}", secret_number);

    println!("请输入你的猜测。");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读取行失败。");

    let guess: u32 = guess.trim().parse().expect("请输入一个正整数！");

    println!("你猜到了：{guess}");
}
