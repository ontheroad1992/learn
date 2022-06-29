fn main() {
    // println!("Hello, world!");
    let s1 = String::from("hello, world");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 必须是可变的值才是使用指针作为引用
    let mut s = String::from("hello");

    change(&mut s);
    println!("s is: {}", s);

    ownership();

    ownership1();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// 引用的值，没有所有权，所以无法修改
// fn change(some_string: &String) {
//     some_string.push_str(", herman")
// }

fn change(some_string: &mut String) {
    some_string.push_str(", herman")
}

fn ownership() {
    let mut s = String::from("hello world");
    {
        let r1 = &mut s;
        r1.push_str(", string");
        println!("r1: {}", r1)
    }

    let r2 = &mut s;
    r2.push_str(", herman");
    println!("r2: {}", r2);

    println!("s: {}", s)
}

fn ownership1() {
    let mut s = String::from("hello2");

    let r1 = &s;
    let r2 = &s;
    // r1.push_str("string"); 没有加 &mut 证明是不可变的
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    r3.push_str(",string");
    println!("s is {}", s);

    // let reference_to_nothing = dangle();
    // println!("reference_to_nothing is {}", reference_to_nothing);
}

// 指针不允许离开当前的作用域
// fn dangle() -> &String {
//     let s = String::from("hello3");

//     &s
// }
