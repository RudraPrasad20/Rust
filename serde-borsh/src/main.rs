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
println!("{}",an);
}

fn print_bigger<'a, 'b>(s1: &'a String, s2: &'b String) -> &'b String {
    
}
