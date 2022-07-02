#[derive(Debug)]
enum User {
    Name(String),
    Age(i32),
    Float(f64),
}

pub fn test() {
    println!("Hello, world!");
    // let v: Vec<i32> = Ve&&&c::new();

    let mut v = vec![1, 2, 3];

    v.push(4);

    let v = v;
    let third = &v[2];
    println!("the third element is {:?}", third);

    // 确保数值是有效的， get 返回时 Option 因此可以被 match 使用
    // 回忆一下 match 的用法， 它类似于 switch
    match v.get(2) {
        Some(third) => println!("this third element is {}", third),
        None => println!("there is no third element"),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // 在相同作用域中不能同时存在可变和不可变
    // first 会被使用，但是 push 可能会生成新的内存空间，会影响 first 的借用
    v.push(6); // error

    println!("vec i : {:?}", v);

    // println!("the first element is : {}", first);

    for i in &v {
        println!("{}", i)
    }

    for i in &mut v {
        *i += 50
    }
    println!("vec i : {:?}", v);

    let row = vec![
        User::Name(String::from("herman")),
        User::Age(30),
        User::Float(1.2),
    ];
    println!("row: {:?}", row);
}
