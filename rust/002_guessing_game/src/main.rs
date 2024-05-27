use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜猜正整数 1 ~ 100 ！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("秘密号码是：{}。", secret_number);

    loop {
        println!("请输入你的猜测。");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取失败。");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("你的猜测：{guess}。");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小 ！"),
            Ordering::Greater => println!("太大 ！"),
            Ordering::Equal => {
                println!("你赢了 ！");
                break;
            }
        }
    }
}
