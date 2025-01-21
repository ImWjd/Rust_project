// struct的方法
// 方法和函数类似：fn关键字、名称、参数、返回值
//方法与函数的不同之处
//（1）方法是struct（或enum、trait对象）的上下文中定义
//第一个参数是self，表示方法被调用的struct实例
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

//定义Rectangle一个方法
//在impl块里定义方法，方法的第一个参数可以是&self，也可以是获得其所有权或可变借用，和其他采纳数一样，每个struct运行具有多个impl块
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
    // 关联函数,在impl块里定义不把self作为第一个参数的函数叫做关联函数，例如String::from,::符号可以用于关联函数或者模块创建的命名空间
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}
fn main() {
    //使用关联函数
    let s = Rectangle::square(20);

    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };

    let rect2 = Rectangle {
        width: 40,
        length: 55,
    };
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect2.can_hold(&rect1));
    println!("矩形面积：{}", rect.area())
}
