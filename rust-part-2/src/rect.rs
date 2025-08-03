pub struct Rect {
    pub width: f32,
    pub height: f32,
}

pub struct Point <T> {
    pub x: T,
    pub y: T,
}

impl Rect {
    pub fn area(&self) -> f32 {
        return self.width * self.height;
    }
    pub fn another(int: i32) {
        println!("This is another test");
        println!("The value is: {}", int);
    }
}