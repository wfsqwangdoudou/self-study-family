use std::io;

fn main() {
    println!("猜猜数字！");

    println!("请输入你的猜测。");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读取行失败。");

    println!("你猜到了：{guess}");

    let x = 5;
    let y = 10;

    println!("x = {} and y + 2 = {}", x, y + 2);
}
