#[allow(unused_imports)]
use std::result;
#[allow(unused)]
fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}", largest);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}", largest);

    let number_list1 = vec![34, 50, 25, 100, 65];
    let result1 = largest(&number_list1);
    println!("The Largest Number Of Number List1 Values Is {}", result1);

    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result2 = largest(&number_list2);
    println!("The Largest Number Of Number List2 Values Is {}", result2);


    // 泛型的作用：提高代码复用能力，处理重复代码的问题。
    // 泛型的本质：是具体类型或其它属性的抽象代替，编写的代码是一种模板，其中有 “占位符”，编译器在编译时会将 “占位符” 替换为具体的类型。
    // 示例：给出了一个函数定义示例 fn largest<T>(list: &[T]) -> T {... } 。T为类型参数，通常是一个字母，CameICase，T是type缩写

    //如果传入的数组是一个字符型，那么函数就无法处理，因为该函数只能处理i32类型
    let char_list = vec!['a','d','r','d','g','m','t'];
    // let result_char = largest_T(&char_list);


    //调用结构体泛型,相同类型
    let integer = Point{x:5,y:1};
    //调用结构体泛型,不同类型
    let integer = Point1{x:5,y:1.2};

}
//使用泛型重新定义的函数
// fn largest_T<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//实现一个结构体的泛型
struct Point<T>{
    x:T,
    y:T,
}

struct Point1<T,U>{
    x:T,
    y:U,
}

//枚举的泛型
enum Option<T> {
    Some(T),
    None
}
enum Result<T,E> {
    ok(T),
    Err(E),
}