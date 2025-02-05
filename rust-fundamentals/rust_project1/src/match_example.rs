#[derive(Debug)]
enum UsState {
    Alabam,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_coin(cion: Coin) -> u8 {
    match cion {
        Coin::Dime => {
            println!("打印？？？？？");
            1
        }
        Coin::Nickel => 5,
        Coin::Penny => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

//使用Option<T>举例子，使用match形式

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

//match 匹配必需穷举出所有可能使用_表示所有未列出的值
fn main() {
    let c = Coin::Quarter(UsState::Alabam);
    println!("{}", value_in_coin(c));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let v = Some(0u8);
    //match 匹配必需穷举出所有可能使用_表示所有未列出的值
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
