fn main() {
    //引用不可变
    let s1 = String::from("hello,world!!!");
    // 把引用作为函数参数的这个行为叫做借用，相当与一个指针变量
    let len = calculate_length(&s1);
    println!("s1:{}", s1);
    println!("The length of '{}' is '{}' ", s1, len);
    //引用可变
    let mut s2 = String::from("hello,world!!!");
    // 把引用作为函数参数的这个行为叫做借用，相当与一个指针变量
    let len = mutcalculate_length(&mut s2);
    println!("s2:{}", s2);
    println!("The length of '{}' is '{}' ", s2, len);
}
//不可变变量
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    // 当变量不可变时如果对借用的变量进行修改
    // s.push_str(",增加内容");//报错：提示借用变量不可修改
    length
}

//可变变量
fn mutcalculate_length(s: &mut String) -> usize {
    // 当变量可变时如果对借用的变量进行修改
    s.push_str(",增加内容"); //可对变量进行修改
    let length = s.len();
    length
}
