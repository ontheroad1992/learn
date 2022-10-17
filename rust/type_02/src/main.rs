use num::Complex;

fn main() {
    println!("Hello, world!");
    // test1() x
    test2();
    test3();
    test4();
    test5();
    // 函数部分
    println!("{}", plus_five(1))
}

// fn test1() {
//     // 涉及精度问题的浮点数比较，直接比较是错误的
//     assert!(0.1 + 0.2 == 0.3)
// }

fn test2() {
    // 浮点数陷阱
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    // let xyz = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits()); // 浮点数直接操作，结果的精度不会准确

    let x = (-42.0_f32).sqrt();
    // 处于安全考虑
    if x.is_nan() {
        println!("未定义的数学行为")
    }
}

fn test3() {
    for i in 1..=5 {
        print!("{}", i)
    }
}

fn test4() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}

fn test5() {
    let x = 1;

    // 类似三元表达式
    let y = if x % 2 == 1 { "odd" } else { "even" };

    println!("y: {}", y)
}

fn plus_five(x: i32) -> i32 {
    x + 1
}

// fn report<T: std::fmt::Debug>(item: T) {
//     println!("{:?}", item);
// }
