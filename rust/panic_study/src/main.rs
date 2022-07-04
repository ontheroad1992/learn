use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    // let f = File::open("hello.txt").unwrap();

    let f2 = read_username_from_file("hello.txt");

    println!("f2: {:?}", f2);

    // let f3: &str = f;
    // println!("line: {:?}", lat_char_of_first_line(f2))
}

fn open_file_or_create() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem createing the file: {:?}", e),
            },
            other_error => {
                panic!("problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn other_fn() {
    // 使用闭包和 unwrap_or_else 方法的例子
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.text").unwrap_or_else(|error| {
                panic!("problem createing the file: {:?}", error);
            })
        } else {
            panic!("problem opening the file: {:?}", error)
        }
    });
}

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    // let mut f = File::open(path);

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // 变行1
    // let mut f = File::open(path)?;

    // let mut s = String::new();

    // f.read_to_string(&mut s)?;

    // Ok(s)

    // 变形2
    // let mut s = String::new();

    // File::open(path)?.read_to_string(&mut s)?;

    // Ok(s)

    // ? 运算符号只能应用于 Result 的返回 或者 Option

    // 最直接的方式
    fs::read_to_string(path)
}

fn lat_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
