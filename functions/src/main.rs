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

    let num = 3;
    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if condition is true return 5 otherwise return 6
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // we cant do this - it has to be of same type
    // let number = if condition { 5 } else { "six" };
    println!("The value of number is: {number}");

    // loop -
    // print from number 0 to 10
    let mut numb = 0;
    loop { 
        println!("number is {numb}");
        // if we will add a continue here, it will run like a infinite loop, it will always print 6
        // if numb == 6 {
        //     continue;
        // }

        // if numb is 10, dont run anymore
        if numb == 10 {
            break;
        }
        numb = numb + 1;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        // it will return 10 * 2 = 20 and break the loop
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut cnt = 0;
    // it has 2 loops, the first one is inner loop and second one is outer loop
    // giving a perticular name to the outer loop - 'counting_up - need to give a single ' at the first
    // why? - we can break the outer loop from the inner loop
    'counting_up: loop {
        println!("count = {cnt}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // breaking inner loop
                break;
            }
            if cnt == 2 {
                // breaking the outer loop here
                break 'counting_up;
            }
            remaining -= 1;
        }

        cnt += 1;
    }
    println!("End count = {cnt}");

    // while loop and arrays :
    // if we had to print all the index values we can do this:
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // also we can print all the index value by another way , without while:
    // it prints all the elements in a
    for ele in a {
        println!("element value is {ele}")
    }

    // print number between 1 to 5 in reverse form
    for number in (1..5).rev() {
        println!("{number}!");
    }
    // it'll print 4,3,2,1
    println!("LIFTOFF!!!");

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