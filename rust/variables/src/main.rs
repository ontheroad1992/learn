fn main() {
    // mut let 的可变性
    // let mut x = 5;
    // p&rintln!("the value of x is:{}", x);
    // x = 6;
    // println!("the value of x is:{}", x)

    // let 的作用域
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
}
