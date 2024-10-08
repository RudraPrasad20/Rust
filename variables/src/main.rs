// Variables and Mutability

// in const type, need to declear the name in CAPITAL
// let cant be used globally, only const can declear globally
// also if you are using const , you need to declear the size by yourself
// in let by default it declears the type according to the value, but in const you need to declear it
const OUTSIDEOFFN: u32 = 9090;
fn main() {
    // mut -> can change
    // immut -> can't change

    let mut x = 5;
    println!("The value of x is: {x}");

    // if you are using mut, you dont need to declear let, otherwise need to declear let
    x = 6;
    println!("The value of x is: {x}");

    // const value cant be changed, value cant be changed
    // you can also declear & use the const value globally outside of the function at the top
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("in sec: {THREE_HOURS_IN_SECONDS}");

    // printing from global const
    println!("outside of fn: {OUTSIDEOFFN}");

    // value is 5
    let xy = 5;
    // value is 6 -> 5 + 1
    let xy = xy + 1;
    {
        // value is 12 -> 6 * 2
        let xy = xy * 2;
        println!("The value of x in the inner scope is: {xy}");
    }
    // it is outside of the obj so it will print the previous value of xy -> 6
    println!("The value of x is: {xy}");

    // shadowing ->
    // you can change the value of it but you cant change the type of it, as we did just above it
    // ex - here we decleard a let value of 5, which is mut means it can be changed
    // but if we change the xyz value to true or false it will give error, because it has decleard before as i32 & now we are trying to set that as bool
    // if we do let again after that then it will print the latest one, the top one would not print , that will be skipped that is called shadowing

    let xyz = 5;
    println!("first value : {xyz}");

    // after uncomment this it will show error
    // you cant change the value again, it is trying to set the xyz value to a bool which is not possible, it is decleard as i32 before (5)
    // and if you will change the value to a int, let suppose 3, it will again show error, you cant change it because it is immutade by default, you need to declear as mut to the previous value

    //   xyz = true;
    //  println!("middle value : {xyz}");

    let xyz = false;
    println!("last value : {xyz}");

    // Tuple:
    // it declears multiple values in a single line
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // also we can declear the tuple like this:
    // it is just like our interface in ts
    let tup = (500, 6.4, 1, true);
    let (x, y, z, e) = tup;

    println!("The value of y is: {x}");

    // also you can define at the top & then use those value one by one, think it like an array
    // it starts from 0 - index no
    let name: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = name.0;

    let six_point_four = name.1;

    let one = name.2;
    println!("here is your tup value: {five_hundred}");

    // Array:
    // to define the type: [type, size]
    let a = [1, 2, 3, 4, 5];

    // to access a single value: access them with index number
    let first = a[0];
    let second = a[1];

    // if you are writing a single value multiple time inside of an array, you should write like this:
    // similar to writing 3 - 5 times
    let a = [3; 5];

}
