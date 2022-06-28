use std::io;

fn main() {
    // mut let 的可变性
    // let mut x = 5;
    // p&&&rintln!("the value of x is:{}", x);
    // x = 6;
    // println!("the value of x is:{}", x)

    // let 的作&用域
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); // 12
    }

    println!("The value of x is: {}", x); // 6

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces len: {}", spaces);

    // let mut spaces = "   ";
    // spaces = spaces.le(); // error 前后的类型不同

    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess is: {}", guess);

    let sum = 5 + 10;
    println!("sum: {}", sum);

    let difference: f32 = 90.2 - 2.1;
    println!("difference: {}", difference);
    // let result: f32 = difference;
    // println!("difference: {}", result);

    let product = 4 * 30;
    println!("product: {}", product);

    let quotient: f32 = 56.7 / 10.0;
    let floored = 2 / 3;
    println!("quotient: {}", quotient);
    println!("floored: {}", floored);

    // 取余
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    {
        // 元组类型
        let tup: (i32, f64, u8) = (500, 9.2, 8);
        println!("tup: {}", tup.0);

        let (x, y, z) = tup;
        println!("tup: {} {} {}", x, y, z)
    }

    {
        // 数组
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        println!("arr: {}", a[1])
    }

    {
        // 无效的数组元素访问
        let a = [1, 2, 3, 4, 5];
        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("index entered was not a number");

        let element = a[index];

        println!(
            "The value of the element at index {} is: {}",
            index, element
        )
    }
}
