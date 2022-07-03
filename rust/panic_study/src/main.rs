use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.txt").unwrap();
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
