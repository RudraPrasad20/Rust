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
    println!("{sm}")
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
