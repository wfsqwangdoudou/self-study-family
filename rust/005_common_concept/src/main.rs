fn main() {
    let mut count = 0;
    let result_counting_up = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        let result = loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break remaining;
            }
            if count == 2 {
                break 'counting_up count;
            }
            remaining -= 1;
        };
        println!("result break = {result}");

        count += 1;
    };
    println!("result_counting_up break = {result_counting_up}");

    println!("End count = {count}");
}
