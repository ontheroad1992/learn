mod front_of_house {
    // 默认私有的，添加 pub 才能变成公有的
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn server_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    use crate::front_of_house::serving;

    // 结构体成员默认私有的
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 枚举成员是默认公开的
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::eat_at_restaurant();
        serving::server_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    hosting::add_to_waitlist();
    hosting::seat_at_table();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I would like {} toast please", meal.toast);

    // meal.seasonal_fruit // error 没有公开的，无法 find

    let order1 = back_of_house::Appetizer::Soup;
}
