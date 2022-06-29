fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello world");
    println!("s : {}", s);

    // let word& = first_word(&s[..]);

    // 这里的直接传入 &s 等价于 &s[..]
    let word = first_word(&s);

    // s.clear(); // ”“

    println!("word : {}", word);
    println!("s : {}", s);

    s = String::from("herman");
    println!("s : {}", s);

    let s1 = String::from("hello world");

    // let len = s.len();

    let hello = &s1[0..5]; // 加 & 意思是对存储的内容的指针空间做操作，完成 s 中还包含 len capacity， &能指向 ptr 内容区域
    let world = &s1[6..];
    println!("slice1: {}, slice2: {}", hello, world)
}

// 返回一个 slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    println!("123213213213 {} str: {}", s.len(), s);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn second_word(s: &String) -> (usize, usize) {}
