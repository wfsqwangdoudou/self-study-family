fn main() {
    let x = plus_one(6);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    if x > 5 {
        return -x;
    }
    return x + 1;
}
