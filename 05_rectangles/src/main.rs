#![allow(unused)]
// debug is a derivable trait - explicitly opt in to make that debug trait {:?} available for our struct.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated functions that is not a method (does not take &self and don’t need an instance of the type to work with)
    // accessed using path
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // Associated functions whose first parameter is named self are called methods
    // accessed using dot
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // println!("rectangle {:#?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(20 * scale),
        height: 80,
    };
    dbg!(&rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("can rect1 hold rect2? {}", rect1.can_hold(rect2));
}
