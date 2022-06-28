fn main() {
    println!("Hello, world!");

    // let y = (let x = 1)&&;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    // 单个字符用 ‘’
    another_function(5, 'h', "xxxx");

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(10);
    println!("plus_one x: {}", x)
}

fn another_function(x: i32, unit_label: char, strs: &str) {
    // println!("another_function")
    println!("the value of x is: {} {} {}", x, unit_label, strs)
}

fn five() -> i8 {
    100
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
