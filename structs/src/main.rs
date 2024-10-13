// Structs are similar to tuples - similar to interface in TS
// declearing a  strurct: (always outside of the main function)
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
// METHODS:-
// impl - implement
// now we can remove the calculate_rectangle function down below, it is similar to that
// creating fn within that impl, passing &self as ref and expecting it will return u32
impl Rectangle {
    fn calc_area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Associated Functions - 
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// tuple structs :
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // if you want to write another user similar to user1 with a single line cahnge, you can do this
    // although we had a better solution for this, we will be writing just after this
    // user1.active -> it is refering to user1, the value will be same as user1
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // if you want to write the same user1 date with only 1 or 2 changes you can do this:
    // here we are changing the email only and rest will be same as user1
    // so we dont need to write the whole data again as we wrote before
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // to update the user, make sure to mut the user
    user1.email = String::from("anotheremail@example.com");

    // calling build_user function
    // dont call via name like email or username, directly update the data
    let another_user: User = build_user(String::from("email@gmail.com"), String::from("value"));

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // calculate the area of a rectangle
    let rect: Rectangle = Rectangle {
        height: 100,
        width: 100,
    };
    let rect2: Rectangle = Rectangle {
        height: 100,
        width: 100,
    };
    println!("can rec1 hold rect2: {}", rect2.can_hold(&rect));
    println!("can rec1 hold rect2: {}", rect.can_hold(&rect2));

    // if you are using methods, no need to use this :
    // let area = calculate_rectangle(&rect);
    // implimenting debug here, in this line rect is showing error but we can debug this by using :? and using debug syntax at the top
    // println!(
    //     "The area of the rectangle {:#?} is {} square pixels.",
    //     rect, area
    // ); // use this if you are using the normal way of couniting area - using structs
    println!(
        "The area of the rectangle {:#?} is {} square pixels.",
        rect, rect.calc_area()
    ); // we can access the same rectangle value by doing this 
    // println!("rect1 is {rect:?}");
    // dbg!(&rect);

    // calling associated function:
    // both height and width will be 3
    let sq = Rectangle::square(3);
}

// to use struct
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// calculating area of a rectangle
// it will work without & (ref) but it is good to use &
fn calculate_rectangle(rec: &Rectangle) -> u32 {
    let ans = rec.height * rec.width;
    ans
}

// {:?} -> debug | {:#?} -> pretty debug
// #[derive(Debug)] - use it at just before the definded struct
// dbg!(&rect); -> we can also use this to debug but it takes ownership

// METHODS:-
// defining a function within the struct it self is called method
// what changes we did to calculate the area of a rectangle:
// 1. define a impl outside of the main fn
// 2. remove the function - here calculate_rectangle
// 3. remove the let area -> let area = calculate_rectangle(&rect);
// 4. to print - println!("The area of the rectangle {:#?} is {} square pixels.", rect, rect.calc_area()); 
// we can call the method syntax like this: 
// rect.calc_area() - rect - name | calc_area() - function

