struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someon&e@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 因为是不可变的，所以无法修改属性值
    // user1.email = String::from("herman@example.com") // error

    let mut user1 = User {
        email: String::from("someone@example.com"),
        ..user1
    };
    // 只有可变的，才可以修改属性，且属性值不能单独设置不可变
    user1.email = String::from("herman@example.com");
    user1.username = String::from("herman");

    let user2 = build_user(user1.email, user1.username);

    // println!("email: {}, username: {}", user1.email, user1.username); // error
    println!("email: {}, username: {}", user2.email, user2.username);
    println!(
        "active: {}, sign_in_count: {}",
        user1.active, user1.sign_in_count
    );

    let white = Color(255, 255, 255);
    println!("rgba: {}, {}, {}", white.0, white.1, white.2)
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
