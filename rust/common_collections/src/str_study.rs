pub fn test() {
    // let mut s = String::new(&)&;
    let data = "inital contents";

    // to_string 使得 str 转换为了 String
    let mut s = data.to_string();

    s.push_str(" test");
    // let t = s;
    println!("s: {}", s);

    let s1 = "hello".to_string();
    let s2 = String::from("world");
    // 为什么使用借用，因为这是 s1 的结构体中的方法去加
    // fn add(self, s: &str) -> String {
    // s1 因为没有使用借用，所以它的所有权被转移
    let s3 = s1 + " " + &s2;
    println!("s3: {}", s3);

    let s4 = format!("{} {}", s3, "herman");
    println!("s4: {}", s4);

    // 这是因为 utf-8 中并不是所有的字符都是 1 个字节
    println!("index 4 = {}", &s4[3..4]);

    for c in s4.chars() {
        print!("{} ", c)
    }

    println!("");

    for b in s4.bytes() {
        print!("{} ", b)
    }
}
