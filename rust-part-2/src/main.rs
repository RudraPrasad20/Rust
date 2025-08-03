use std::fs;

use rect::Rect;

use crate::rect::Point;
pub mod rect;

fn main() {
    let str = Rect {
        width: 30.0,
        height: 50.0,
    };
    println!("{} {}", str.height, str.width);
    println!("{}", str.area());
    Rect::another(30);

    let inti_one = Point{
        x: 10,
        y: 20,
    };
    let inti_two = Point{
        x: true,
        y: true,
    };

    let contents = fs::read_to_string("a.txt");
    match contents {
        Ok(data) => println!("File contents: {}", data),
        Err(er) => println!("Error reading file: {}", er),
    }
}

// understanding enums in Rust

// enum Shape{
//     circle(f32), // radius
//     rectangle(f32, f32), // width, height
//     square(f32), // side length
// }

// fn main() {
//     let circle_shape = Shape::circle(30.0);
//     let rectangle_shape = Shape::rectangle(20.0, 40.0);
//     let square_shape = Shape::square(15.0);
// }

// fn calculate_area(dir: Shape) -> f32 {
//     match dir {
//         Shape::circle(radius) => {
//             println!("This is a circle with radius {}", radius);
//             return std::f32::consts::PI * radius * radius;
//         },
//         Shape::rectangle(width, height) => {
//             println!("This is a rectangle with width {} and height {}", width, height);
//             return width * height;
//         },
//         Shape::square(side) => {
//             println!("This is a square with side length {}", side);
//             return side * side;
//         },
//     }
// }
