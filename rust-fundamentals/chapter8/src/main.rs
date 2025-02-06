#![allow(unused)]

use std::string;
fn main() {
    //从new函数开始创建字符串
    let mut str_example = String::new();

    //从字符串字面创建String
    let data = "HelloWorld";
    let s_data = data.to_string();
    println!("打印输出s_data数值:'{}'", s_data);
    //该方法也可以直接用字符串字面量：
    let s_data2 = "HelloWorld".to_string();
    println!("打印输出s_data2数值:'{}'", s_data2);

    //使用 String::from 函数来从字符串字面量创建 String,等同于to_string
    let s_data3 = String::from("HelloWorld,s_data3");
    println!("打印输出s_data3数值:'{}'", s_data3);

    //字符串是 UTF-8 编码的，可以包含任何正确编码的数据
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    //String 的大小可以增加，其内容也可以改变，可以使用 + 运算符或format! 宏来拼接 String 值
    //使用 push_str 和 push 附加字符串
    //push_str 方法的使用：push_str 方法用于将一个字符串切片（&str）追加到 String 类型的字符串后面，它不会获取参数的所有权
    //push 方法的使用：push 方法用于将一个字符（char 类型）追加到 String 类型的字符串后面
    let mut s_data4 = String::from("hellow,");
    println!("s_data4原始字符串为：{}", s_data4);
    s_data4.push_str("world!!!");
    println!("s_data4扩展后的字符串为：{}", s_data4);

    //将s_data5附加给s_data4
    let mut s_data5 = ",I'm wjd";
    s_data4.push_str(s_data5); //s_data4并未获取到s_data5的所有权
    println!("s_data4原始的字符串为：{}", s_data4);
    println!("打印s_data5原始的字符串为：{}", s_data5);

    //使用 push 将一个字符加入 String 值中
    let mut s_data6 = String::from("使用push进行附加字符串");
    s_data6.push('s');
    println!("s_data6扩展后的字符串为：{}", s_data6);

    //使用 + 运算符或 format! 宏拼接字符串,将两个已知的字符串合并在一起
    //+运算符使用了add函数
    // fn add(self, s: &str) -> String{} //add只能将&str与String相加
    let s1 = String::from("hellow,");
    let s1_clone = s1.clone(); //使用clone就可以防止所有值的转移
    let s2 = String::from("wrold!!!"); //&s2 的类型是 &String被强转（coerced）成&str,使用了解引用强制转换（deref coercion）技术,add获取了self的所有权，因为self没有使用&,所以将s1的所有权移动到add调用中
    let s3 = s1_clone + &s2;
    // println!("打印s1：{}",s1);//s1报错，所有权被转移

    println!("打印s2：{}", s2);
    println!("打印s3：{}", s3);

    //级联多个字符串
    let a1 = String::from("tic");
    let a2 = String::from("tac");
    let a3 = String::from("toe");

    let a4 = a1 + "-" + &a2 + "-" + &a3;
    println!("级联多个字符串{}",a4);

    //对于更为复杂的字符串，可以使用format!宏
    let b1 = String::from("tic");
    let b2 = String::from("tac");
    let b3 = String::from("toe");

    let b4 =format!("{}-{}-{}",b1,b2,b3);
    println!("使用format!宏{}",b4);

    //索引字符串,在rust语言当中无法通过引用字符串中的单独字符来进行索引
    // let b4_index = b4[0];




}
