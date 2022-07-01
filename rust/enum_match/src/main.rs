use rand::Rng;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin_value = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("coin_value: {}", coin_value);

    let five = Some(5);
    let six = plus_one(five);
    println!("six: {:?}", six);
    let none = plus_one(None);
    println!("none: {:?}", none);

    // let dice_rool: u32 = 9;
    let dice_rool = rand::thread_rng().gen_range(1..10);

    match dice_rool {
        7 => remove_fancy_hat(),
        3 => add_fancy_hat(),

        // other => move_player(other),
        // 必须有一个接受穷尽的值
        // _ => reroll(),
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u32) {
    println!("num_spaces: {}", num_spaces)
}

fn reroll() {}
