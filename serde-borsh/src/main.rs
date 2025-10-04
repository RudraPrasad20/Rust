// Serde:

// update cargo.toml:
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let point = Point { x: 1, y: 2 };

//     // Convert the Point to a JSON string.
//     let serialized = serde_json::to_string(&point).unwrap();

//     // Prints serialized = {"x":1,"y":2}
//     println!("serialized = {}", serialized);

//     // Convert the JSON string back to a Point.
//     let deserialized: Point = serde_json::from_str(&serialized).unwrap();

//     // Prints deserialized = Point { x: 1, y: 2 }
//     println!("deserialized = {:?}", deserialized);
// }

// brosh:

// use borsh::{BorshDeserialize, BorshSerialize};

// #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
// struct A {
//     x: u64,
//     y: String,
// }

// fn main() {
//     let a = A {
//         x: 3301,
//         y: String::from("Hello, Borsh!"),
//     };

//     let mut v = Vec::new();

//     let ser = a.serialize(&mut v).unwrap();

//     println!("Original: {:?}", v);

//     let dec = A::try_from_slice(&v).unwrap();
//     println!("Deserialized: {:?}", dec);
// }


// lifetimes:


fn main() {
let u1 = String::from("rudra");
let an: &String;
{
    let u2 = String::from("");
    an = print_bigger(&u1, &u2);
}
// if u2 is longer than u1, it will cause dangling pointer
// when we print an, it will crete error, otherwise the written code will run fine by using lifetimes
// println!("{}",an);
}
// here is how we will get lifetime error, 
// there is chance of u2 is longer than u1, we are checking for longer one, if u2 becomes longer, it will create error 
// because , notice u2 has small lifetime, it is defined in a scope

// by using referance - in simple form
// fn print_bigger(s1: &String, s2: &String) -> &String {
// if s1.len() > s2.len() {
//     return s1;
// } else { 
//     return  s2
// }
// }

// using lifetime
fn print_bigger<'a, 'b>(s1: &'a String, s2: &'b String) -> &'b String {
    &s2
}
