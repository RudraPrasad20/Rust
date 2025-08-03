use chrono::{Utc, Local};

enum Add {
    Sum(i32, i32),
}
fn main() {
    println!("Hello, world!");

    let today_utc = Utc::now();
    let today_local = Local::now();
    println!("Today is: {}", today_utc.format("02/04/2017 12:50"));
    println!("Local time is: {}", today_local.format("02/04/2017 12:50"));

    let sum = sum_fn(2, 4);
    println!("Sum is: {}", sum);
}

fn sum_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn do_something() {
    let add = Add::Sum(2, 3);
    match add {
        Add::Sum(a, b) => println!("Sum is: {}", a + b),
    }
}