#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}
fn main() {
    let w = 30;
    let h = 40;
    println!("{}", area(w, h));

    // 声明一个长方形
    let rect = (30, 50);
    println!("{}", area1(rect));

    let rect = Rectangle {
        width: 30,
        length: 5,
    };
    println!("{}", area2(&rect));

    println!("{:?}",rect);//直接打印结构体会报错,:?是普通打印，:#?是美化打印
}

fn area(width: u32, length: u32) -> u32 {
    width * length
}
// 使用元祖类型
fn area1(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}
// 使用结构体形式
fn area2(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}
