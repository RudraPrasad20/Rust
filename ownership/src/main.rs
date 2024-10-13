fn main() {
    // the s is valid within the scope only, we cant access the s outside the scope
    // {} - scope
    // it is stored in 'stack'
    {
        let s = "hello";
    }

    // storing s in 'heap'
    // if we will use String::from - it is growable string - also dynamic
    // we can push something or pop something in case of String::from, it is not possible in the simple &str type
    // if we want to upgrade anything, we use String::from
    // to push something we need to declear mut
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("ans is : {s}");

    // intersting:
    let mut x = 5;
    let y = x;
    x = 10;
    println!("x is : {x} & y is : {y}");
    // here what you will notice is:
    // when x is 5 y is also 5 - you can see this with a println just before changing the x value
    // then when we cahange the x value to 10 it doesn;t changes because it is just copying the x

    // but we cant copy here, because it is dynamic and it is stored in 'heap', and copying something in heap is very expensive
    let s1 = String::from("hello");
    // we can't do this because it is directly copying the s1. you can see the error in println()
    // let s2 = s1;
    // insted we can do this, with using a clone function
    let s2 = s1.clone();
    // if you will try to print here, it will show error
    println!("s1 is: {s1} & s2 is :{s2}");

    // this is called double free error -
    // when we decleared s1 it is setat a perticular index and after all in rust it clears that automatically
    // so when we declear s1 it is set to an index and it has been cleared then we tried to declear the s2 so it is not possible
    // both are heading to a single index
    // rust rule: you might think when we declear s1 and s2 even after we call s2=s1 why cant we acces s1.
    // when you put s1 into s2 it invalidates s1, s2 takes the place of s1 - index - now s2 is the owner

    let num = 10;
    let result = add(num + 10);
    println!("num: {num} & result: {result}");

    let str = String::from("value");
    in_ownership(str);
    // but you cant do this -
    // println!("{str}")
    // because it moved to the s of in_ownership function, and as we seen before we cant access the 1st one after shift
    // it is similar to moving from s1 to s2, here it moving from str to s

    // but we can do this -
    // it calls a function which returns string
    let sr = String::from("hii there");
    let sm = takes_ownership();
    println!("{sm}");

    // see in this example 1st we created a name nme which will be the input for calculating the length
    // then we called length_calculator function
    // but after calling the function we cant do println the nme, because the ownership has been moved to the length_calculator
    // to println the nme what we can do is:-
    // we decleared another name which takes the name and length & nme passes to length_calculator function
    // snm is the name or string and len is length
    let nme = String::from("calculating length");
    let (snm, len) = length_calculator(nme);
    // length_calculator(nme);
    // println!("{nme}");
    println!("the name is {snm} & length is {len}");

    // REFERENCES :-
    // String - ownership
    // &String - reference | * - dereference
    // here s is the reference, sone is the owner
    // we did mut everyware because we passed the &mut at the calculate_length function
    let mut sone = String::from("hello"); // sm : original pointer
    let len = calculate_length(&mut sone);
    println!("The length of '{sone}' is {len}.");

    // if we will do this it will show error because we are trying to mut the original owner s
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);
    // if we will print the r3 only, then it will work because the r1 and r2 has created and ened there only in the same line, we never used those
    // println!("{}, r3);

    // but we can mut the st here, how?
    // see here we moved the println to the top of r3, and the ownership lasts untill it is used somewhere
    // here we used the r1 and r2 as println so the ownership has been gone , now we can mut the r3
    let mut st = String::from("hello");

    let r1 = &st; // no problem
    let r2 = &st; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point
    let r3 = &mut st; // no problem
    println!("{r3}");

    // SLICE:-
    let mut sli = String::from("hello world");
    let word = first_word_slice(&sli); // word will get the value 5 because the first word is hello
    sli.clear();
    // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // [starting_index..ending_index]
    let hello = &sli[0..5]; // word hello - index 0 to 5
    let world = &sli[6..11]; // word world - index 6 to 11

    let sliceone = &sli[0..2];
    let slicetwo = &sli[..2];

    let lent = sli.len();

    let slicethree = &sli[3..lent];
    let slicefour = &sli[3..];

    let slicefive = &sli[0..lent];
    let slicesix = &sli[..];

}

fn add(n: i32) -> i32 {
    n + 10
}

fn in_ownership(s: String) {
    println!("in ownership: {s}");
}

fn takes_ownership() -> String {
    let s: String = String::from("This is a string from gives ownership"); // some_string comes into scope
    s
}

fn length_calculator(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

// &String - pointer referancing to index
// String - move
// &String - borrowing
// we cant mutate the s here, because we have borrowed the value and we dont have the permission to mut, it is a ref
// but we can change the value at the top, because that is the owner - sone
fn calculate_length(s: &mut String) -> usize {
    // if we will try to push something - it will show error, for that we have to change the mut the &String - &mut String
    s.push_str(", world");
    s.len() // to access length
}
// we can't take multiple mutable referances

// Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

//  SLICE TYPE:-
//  write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

fn first_word_slice(s: &String) -> usize {
    //  we’ll convert our String to an array of bytes using the as_bytes method.
    let sl = s.as_bytes();
    // iteraing with for loop in sl
    for (i, &item) in sl.iter().enumerate() {
        // finding space in the byte
        if item == b' ' {
            return i;
            // return &s[0..i]; // same , returns 1st word
        }
    }
    s.len()
    // &s[..] //same, retruns length
    
}
