#[derive(Debug)] // use derive annotation to use Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // automatic referencing and dereferencing
    println!(
        "The area of the rectangle is {} square pixels.",
        &rect1.area()
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        (&rect1).area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // calling associated function within Rectangle struct's namespace
    let sq = Rectangle::square(3);
    println!("{:#?}", sq)
}

fn area(rectangle: &Rectangle) -> u32 { // rectangle borrows the rect1 struct rather than takes ownership of it.
    rectangle.width * rectangle.height
}
