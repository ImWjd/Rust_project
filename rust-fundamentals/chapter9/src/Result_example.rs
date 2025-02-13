use core::panic;
use std::{fs::File, io::{self, ErrorKind, Read}};
#[allow(unused)]
#[warn(unused_imports)]
fn main() {
    // Rust 将错误组合成两个主要类别：可恢复错误（recoverable），例如：文件未找到，和不可恢复错误（unrecoverable），例如项目bug​。
    //对于可恢复的错误rust提供 Result<T,E>,对于不可恢复的提供了：panic!宏
    // panic!宏会打印一个错误信息，展开（unwind）、清理调用栈(Stack)、退出程序
    //默认情况下，当panic发生：（1）程序调用栈（工作量大）rust沿着调用栈往回走，清理每个遇见的函数当中的数据。（2）立即终止调用栈：不进行清理、直接停止程序，内存需要操作系统进行清理。
    //想让二进制文件更小可以设置从“展开”改为“终止”操作；在cargo.toml中适当的profile部分设置，设置panic='about'
    // [profile.release]
    // panic = 'abort'
    // 通过设置环境变量RUST_BACKRACE可得到回溯信息，为了获取带有调试信息的回溯，必须启用调试符号（不带--release）

    // panic!("crash and burn,调用panic！宏");

    // let v= vec![1,2,3,4];

    // v[99];

    // Result枚举与可恢复的错误
    enum Result<T, E> {
        OK(T),  //操作成果情况下，Ok变体里返回的数据的类型
        Err(E), //操作失败情况下，Err变体里返回的错误的类型
    }
    //例子
    let f_match = File::open("hello.txt");
    //和Option枚举一样，Result及其变体也是有Prelude带入作用域
    // 使用match来返回错误信息

    // let f_match =match f_match {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("文件打开出现问题{:?}", error);
    //     }
    // };


    // 使用 match 表达式来处理文件打开的结果
    let f_match = match f_match {
        // 如果文件成功打开，将打开的文件对象（file）作为结果返回
        Ok(file) => file,
        // 如果文件打开失败，处理错误情况
        Err(error) => match error.kind() {
            // 检查错误类型是否为文件未找到（ErrorKind::NotFound）
            ErrorKind::NotFound => {
                // 如果文件未找到，尝试创建该文件
                // File::create 函数返回一个 Result<File, io::Error> 类型的值
                // 如果文件成功创建，返回 Ok(File)；如果创建失败，返回 Err(io::Error)
                match File::create("hello.txt") {
                    // 如果文件成功创建，将创建的文件对象（fc）作为结果返回
                    Ok(fc) => fc,
                    // 如果文件创建失败，使用 panic! 宏打印错误信息并终止程序
                    Err(e) => panic!("无法创建未找到的文件{:?}", e),
                }
            }
            // 处理其他类型的错误
            other_error => {
                // 使用 panic! 宏打印错误信息并终止程序
                panic!("无法打开文件{:?}", other_error);
            }
        },
    };

    // 第二种方法
    let f_match2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("文件创建错误：{:?}", error);
            })
        } else {
            panic!("文件打开错误：{:?}", error);
        }
    });
    // unwarp是match表达式的一个快捷方式，如果Result结果是OK则返回OK里面的值、如果结果是Err则调用panic!宏

    let f = File::open("hello.txt").unwrap();//unwarp函数的返回结果不支持自定义

    // expect和unwarp类似，但是可以指定错误信息
    let f1 = File::open("hello.txt").expect("无法打开文件");

    let result1 = read_username_from_file(); 

}


fn read_username_from_file() -> Result<String,io::Error>{
    let mut f =File::open("hello.txt")?;

    // let mut f = match f{
    //     Ok(file) =>file,
    //     Err(e)=>return Err(e),
    // };
    let mut s = String::new();
    f.read_to_string(&mut s)?;//?作用域Result与下边match操作是一样的，如果操作成果就是OK，失败则是Err
    Ok(s)//如果代码都操作成果在最后则返回一个Ok
    // match f.read_to_string(&mut s){
    //     Ok(_)=>Ok(s),
    //     Err(e)=>Err(e),
    // }


    // ?与from函数，Trait std::convert::From 上的from函数用于错误之间的转换，被?所应用的错误会隐式的被from函数处理
    //当?调用from函数时，它所接收的错误类型会被转化为当前函数返回类型所定义的错误类型
    //EA -> EB, EA from EB ,在 Rust 中，如果要使用 ? 操作符将错误类型 EB 转换为错误类型 EA，就必须为 EA 实现 From<EB> trait，也就是要实现 impl From<EB> for EA。用于针对不同错误原因，返回同一种错误类型
}