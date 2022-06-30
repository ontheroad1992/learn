#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
