fn main() {
    println!("Hello, world!");

    // calling another function
    // rust runs only one function which is the main function, so we need to call other functions inside main function
    another_function();
    second_function(5, 'k');

    // we cant do things like this in rust, as we do in Js => let x = y = 6, in rust it is not valid
    // let x = (let y = 6);

    // A new scope block created with curly brackets is an expression
    // here k = 4 => 3+1
    let k = {
        let xn = 3;
        xn + 1
    };
    // it will return 4
    println!("The value of y is: {k}");

    is_even(4);
}

// it is in snake case
// _ denotes a space between them
fn another_function() {
    println!("Another function.");
}

// calling the function with a number value and a unit character
// we can take multiple values as props
fn second_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// declearing a function is a statement
// calling a function is an expression
// return -> early

// if you will print this you will get :
// if you will call with a truth value it will return true , if you will call with a false value it will return false
// if condition is true directly return no need to go forther, otherwise skip this and print false
fn is_even(x: i32) -> bool {
  if x % 2 == 0 {
    return true;
}
    println!("it is not true and it is not even");
    false
}

// () -> i32 => expected type and value
// fn plus_one(x: i32) -> i32 {
//     x + 1;
// }