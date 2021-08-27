#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, obj: &Rectangle) -> bool {
        self.width >= obj.width && self.height >= obj.height
    }
}

impl Rectangle {
    // Associated Functions can be called by ::
    // defined by absence of &self
    fn square(size : u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main () {
    let rect1 = Rectangle {
        width: 10,
        height: 10,
    };
    println!("Value: {:#?}", rect1);
    println!("Area: {}", rect1.area());

    let rect2 = Rectangle {
        width: 9,
        height: 11,
    };

    let rect3 = Rectangle {
        width: 8,
        height: 8,
    };

    println!("Can hold: {}", rect1.can_hold(&rect2));
    println!("Can hold: {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(20u32);
    println!("Area: {}", square.area());
}