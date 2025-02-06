// mod front_of_house{
//     pub mod hosting{
//        pub fn add_to_waitlist(){

//         }
//     }
// }
// //在rust当中所有的函数、变量、struct、enum等都是私有的，要想变成公共的需要添加pub
// pub fn eat_at_restaurant(){
//     crate:: front_of_house::hosting::add_to_waitlist();//使用绝对路径形式来调用函数
//     front_of_house::hosting::add_to_waitlist();//使用相对路径形式来调用函数
// }

// //在rust语言中使用super关键字来访问父级模块路径中的内容，类似文件系统中的..
// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();//使用的是相对路径
//         crate::serve_order();//使用的是绝对路径
//     }

//     fn cook_order() {}
// }
// //结构体可设置为公有的，pub放在struct之前，struct是公共的，struct的字段默认是私有的，struct的字段需要单独设置pub来变成共有的
// mod back_of_house{
//     pub struct Breakfast{
//         pub toast:String,//添加pub内部就是公共的，不添加是私有的

//         seasonal_fruit:String,
//     }
// }

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     // 在夏天点一份黑麦面包作为早餐
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // 更改我们想要的面包
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // 如果取消下一行的注释，将会导致编译失败；我们不被允许
//     // 看到或更改随餐搭配的季节水果
//     // meal.seasonal_fruit = String::from("blueberries");
// }

// //如果将枚举设为公有，则它的所有成员都将变为公有
// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

// mod front_of_house{
//     pub mod hosting{
//         pub fn add_to_waitlist(){

//         }
//         fn some_function(){}//私有函数，私有函数在引用之后还是无法使用，遵守私有性规则
//     }
// }
// use crate::front_of_house::hosting;//use引用条目，绝对路径
// // use front_of_house::hosting;//use引用条目，相对路径
// pub fn eat_at_restaurant(){
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// use std::collections::HashMap;
// fn test(){
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }

// //函数同名时可以对函数名进行修改
// use std::fmt::Result;
// use std::io::Result as IoResult;

//使得引用的函数在外部也可以使用,可以将条目引入作用域，该条目可以被外部代码引入到他们的作用域之内
// pub use crate::front_of_house::hosting;

// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering,io};


// use std::io;
// use std::io::Write;
// use std::io::{self,Write};

// use std::collections::*;







