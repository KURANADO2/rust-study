mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn set_at_table() {}
    }

    pub mod serving {
        fn task_order() {}

        pub fn serve_order() {}

        fn task_payment() {}
    }
}

mod back_of_house {

    // struct 变成公有，struct 中所有 field 仍然是私有的，如果想让某个 field 变成共有，则需要为其单独加上 pub
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // enum 变成公有，enum 下所有成员都将变为公有
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        // fix_incorrect_order 和 cook_order 处于同一模块
        cook_order();
        // 相对路径
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

fn eat_at_restaurant() {
    // 绝对路径
    // 因为 eat_at_restaurant 和 front_of_house 是兄弟，所以 front_of_house 不需要加 pub 关键字也可以访问到
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("{}", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;
}

// 绝对路径
// use crate::front_of_house::hosting;
// 相对路径
// use self::front_of_house::hosting;
// 无法直观知道 add_to_waitlist 是在哪里定义的，所以一般不会这么使用
// use crate::front_of_house::hosting::add_to_waitlist;
// 让外部也可以引入
pub use self::front_of_house::hosting;
use std::collections::HashMap;
// use std;
// use std::cmp::Ordering;
// use std::io;
use std::{self, cmp::Ordering, io};
// 将 std::collections 下所有公有项引入作用域（不建议）
use std::collections::*;

fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // 无法直观知道 add_to_waitlist 是在哪里定义的，所以一般不会这么使用
    // add_to_waitlist();
}
