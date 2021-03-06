pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(50, 30);
    let sq = Rectangle::square(10);

    println!("Area of rect1 is {} sq.pxls.", rect1.area());
    println!("Area of rect2 is {} sq.pxls.", rect2.area());
    println!("Area of sq is {} sq.pxls.", sq.area());

    println!("{}", rect1.can_hold(&rect2));
}
