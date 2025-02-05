fn main() {
    let v = Some(0u8);
    //match 匹配必需穷举出所有可能使用_表示所有未列出的值,这是多种情况的形式
    match v {
        Some(3) => println!("three"),
        Some(4) => println!("four"),
        Some(5) => println!("five"),
        Some(6) => println!("six"),
        _ => (),
    }
    //当出现一直匹配模式是可以使用一下两种形式来进行书写
    match v1 {
        Some(3) => println!("three"),
        _ => (),
    }
    // 使用if let更为简洁，放弃了穷举可能，if let可以搭配else使用
    if let Some(3) = v {
        println!("three");
    } else {
        println!("other");
    }
}
