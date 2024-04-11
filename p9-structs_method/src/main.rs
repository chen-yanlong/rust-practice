#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(width: u32) -> Rectangle {
        Rectangle{
            width: width,
            height: width,
        }
    }
}

fn main() {
    let rect = Rectangle{
        width: 20,
        height: 30,
    };

    // `#` for better formating
    println!("{:#?}", rect);
    println!("The area of the rectangle is {}", rect.area());

    let rect2 = Rectangle::square(13);

}
