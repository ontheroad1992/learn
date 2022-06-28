fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true")
    } else {
        println!("condition was false")
    }

    if number != 0 {
        println!("number was something other than zero")
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // error

    println!("The value of number is: {}", number);

    let mut count = 0;
    // 循环标签
    'counting_up: loop {
        println!("-----------------------");
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // 跳出 conting_up 循环
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End Count = {}", count);

    let mut counter = 0;

    // 循环的返回值
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;
    // while 循环
    while number != 0 {
        println!("{}!", number);

        number -= 1;

        // if number == 1 {
        //     break;
        // }
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for item in a {
        println!("the value is: {}", item)
    }

    for item in (1..10).rev() {
        println!("{}!", item)
    }
    println!("LIFTOFF!!!!!!!!!")
}
