use std::collections::HashMap;

fn main() {
    // signed(u) - +ve / -ve
    let number_one: u8 = 20;

    // unsigned(i) - cant be -ve
    let number_two: i8 = -30;

    // float(f) - decimals
    let number_three: f32 = 3.2;

    // char - ''
    let character: char = 'r';

    // string - " "
    let str: &str = "hello";

    // bool - true / false
    let bool: bool = false;

    // Arrays:
    // [type, size]
    let arr: [i8; 4] = [3, 4, 5, 6];

    // print 10 for 5 times in an array
    // [10, 10, 10, 10, 10]
    let second_arr: [i8; 5] = [10; 5];

    // index, length
    println!("array is: {} and length is: {}", arr[1], second_arr.len());

    // debug{:?} - to print arr
    println!("{:?}", second_arr);

    // Tuple:
    let tup: (&str, &str, i8) = ("one", "two", 39);

    // a - one, b - two, c - three
    let (a, b, c) = tup;

    // 1,2,3  - index
    println!("tuple one: {}, two: {}, three: {}", tup.0, tup.1, tup.2);

    // debug - {:?}
    println!("{:?}", tup);
    println!("a: {},b: {},c: {}", a, b, c);

    // Functions:
    let number = 30;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("problem not solved yet...")
    }

    // Mutability:
    // mut - mutable - can change, immutable by default
    let mut thirty = 30;
    thirty = 40;

    // Slice:
    let num_arr = [1, 2, 3, 4, 5];

    // & - reference - not shifting ownership - just a cpoy
    // 3, 4, 5
    let slice_one = &num_arr[2..4];
    // all - 1, 2, 3, 4, 5
    let slice_two = &num_arr[..];
    // 2nd to end - [1..]
    // 1st to 3rd - [..2]

    // Ownership:
    let mut char_string: String = String::from("value");

    // push d
    char_string.push('d');

    // replace hello with hey
    char_string.replace("hello", "hey");

    // Match:
    // must need to declear a _(here only), although we'll use struct / enum
    let coin: i8 = 2;
    match coin {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 | 5 => println!("four or five"),
        6..=9 => println!("between six to nine"),
        _ => println!("default value"),
    }

    let name = String::from("Tiger");
    let color = String::from("blue");
    // we cant simply define the name and color here because we decleared these as String - dynamic
    // calling struct
    let ani = Animal {
        name,
        leg: 5,
        color,
    };
    // printing impl fn
    ani.print_fn();

    let b: Myenum = Myenum::B((5), (4));
    let c: Myenum = Myenum::C { x: 3, y: 3 };
    println!("{:?}: {:?} :{:?}", a, b, c);

    // Vector: 
    let mut vec: Vec<i64> = vec![1,2,3,4,5,6];
    vec.len();
    vec[2];
    vec.push(7);
    vec.remove(4);
    // println!("{}", vec);

    // Map:
    let mut map = HashMap::new();
    map. insert(0, "Hi");
    map.insert(1, "Hi2");
    println!("{:?}", map);


    match map.get(&0) {
        Some (str) => println!("{}", str),
        None => println!("Doesn't exist in map")
    }
    match map.get(&2) {
        Some (str) => println!("{}", str),
        _ => println!("Doesn't exist in map")
    }
    map.remove(&0);
    println!("{:?}", map);
}

// enum:
#[derive(Debug)]
enum Myenum {
    B(i32, i8),
    C { x: i32, y: i32 },
}

// struct:
struct Animal {
    name: String,
    leg: i8,
    color: String,
}

// interface(TS) - trait(RS)
trait Anim {
    // can pass fn which are not defined yet
    // can fight - true/ false || can run -  true by default - but we can change while calling out
    fn can_fight(&self) -> bool;
    fn can_run(&self) -> bool {
        true
    }
}

// impl trait for impl fn
impl Anim for Animal {
    fn can_fight(&self) -> bool {
        false
    }
    fn can_run(&self) -> bool {
        true
    }
}

// impl - is a type of fn
impl Animal {
    fn print_fn(&self) {
        println!("{}", &self.leg)
    }
}

// pub - public, by default - private
pub fn public_fn() {
    // 0, 1, 2, 3
    for i in 0..4 {
        println!("{}", i)
    }
}

// Options<>
// Result<OK, err>