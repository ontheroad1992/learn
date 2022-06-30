#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width() * self.height
    }
    fn width(&self) -> u32 {
        if self.width >= 100 {
            return 100;
        }
        self.width
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn is_bool(&self, area: u32) -> bool {
        self.area() > area
    }
}

// 可以写多个相同名字的 impl 块
impl Rectangle {
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
        "the area of the rectangle is {} square pixels. width: {}, height: {}",
        area(&rect1),
        rect1.width,
        rect1.height
    );

    println!("rect1 is {:#?}", rect1);

    dbg!(&rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(10 * scale),
        height: 50,
    };

    // 只是借用，如果是 rect2 的话，会所有权到此结束
    dbg!(&rect2);

    println!("rect2: {:#?}", rect2);

    let mut rect3 = Rectangle {
        width: 200,
        height: 100,
    };
    rect3.set_width(99);
    println!("rect3 area: {}", rect3.area());
    let area = 20000;
    println!("rect3 area > {}, result: {}", area, rect3.is_bool(area));
    println!("react3 max width: {}", rect3.width());

    let rect4 = Rectangle::square(32);
    println!("rect4 area is : {}", rect4.area());

    let is_bool = Rectangle::is_bool(
        &Rectangle {
            width: 100,
            height: 10,
        },
        100,
    );
    println!("is_bool: {}", is_bool)
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
