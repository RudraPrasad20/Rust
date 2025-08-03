
fn main() {
    // println!("Hello, world!");
    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;

    // let tupi = (500, 6.4, 1);
    // let mupi = tupi.0;
    // println!(" value is: {mupi}");

//     let mut string = "Hello, Rust!";

//     string  = "Hello, Rust!";
//     println!("The value of number is: {string}");

//         let spaces = "   ";
//     let spaces = spaces.len();
//     println!("The value of spaces is: {spaces}");

//     let guess: u32 = "00".parse().expect("Not a number!");
//     println!("The value of guess is: {guess}");

//     let a = [3; 5];
//     println!("The value of a is: {:?}", a);

//     another_fn(33, 'm');
// }

// fn another_fn(nm: i32, nmm: char) {
//     println!("This is another function! and the value is: {nm}, {nmm}");

// let x = 5;
// let x = x + 1;

// let num: () = {
//     let x = x*4;
//     println!("The value of x is: {x}");
// };
// println!("The value of x is: {x}");
// println!("The value of num is:{:?}", {num});

//   let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is {y}");



    // let x = 5;
    // if x == 5 {
    //     println!("x is five");
    // } else {
    //     println!("x is not five");
    // }

    // let x = 5;
    // if x == 5 {
    //     println!("x is five");
    // } else if x == 6 {
    //     println!("x is six");
    // } else {
    //     println!("x is not five or six");
    // }
// loop  {
//     println!("going good")
// }

let mut s = String::from("hello");
 s.push_str("djd");
let stwo = s.clone();
println!("the value of s is: {}", {s});
println!("the value of stwo is: {}", {stwo});

let x = 5;
let y = x;
let z = x.clone();
println!("the value of x is: {}", {x});
println!("the value of y is: {}", {y});
println!("the value of z is: {}", {z});

}