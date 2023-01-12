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

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: dbg!(20 * scale),
        height: 40,
    };

    let square = Rectangle::square(3);
    let area = rect1.area();
    let contained = rect1.can_hold(&rect2);


    dbg!(square);
    dbg!(area);
    dbg!(contained);
    println!("{:?}", rect1);
}
