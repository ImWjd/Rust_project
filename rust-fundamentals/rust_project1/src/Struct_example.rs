// 定义一个struct，使用struct关键字对整个struct命名
//在花括号内，为所有字段（Field）定义名称和类型,需要为字段进行复制
// struct实例拥有其所有的数据，只要struct实例有效，那么里面的字段数据也是有效的，struct可以存放引用但是需要使用生命周期
struct User{
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}

// struct作为函数的返回值
fu build_user(email:String,username:String)->User{
    User{
        username:String,
        email:String,
        sign_in_count:u64,
        active:bool,
    }
}

//tuple struct整体有个名，但里面当中的元素没有名，适用于：给整个tuple起名，并不想让他不同于其他tuple，不需要给每个元素进行起名
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
let black = Color(0,0,0);
let  origin = Point(0,0,0);

// 空结构体Unit_Like Struct，没有定义任何字段的struct，适用于在某个类型上实现某个trait，但是不想在内部存储数据



fn main(){
    println("hello,world!!!");
    //实例化User实例
    let user1 = User{
        username:String::from("wjd"),
        email:String::from("1594803224@qq.com"),
        sign_in_count:String::from("64000.1"),
        active:true,
    }
    //取出User中的某个值,进行重新复制
    user1.username = String::from("123456789@qq.com");
    // 如果struct实例是可变的那么实例中的所有字段都是可变的，不允许单独声明
    // struct更新语法，基于某个实例来创建一个新的struct实例
    //方法1
    let user2 = User{
        username:String::from("wjduser2"),
        email:String::from("1594803224@qq.com"),
        sign_in_count:user1.sign_in_count,
        active:user1.active,
    }
    //方法2,效果与方法1等同
    // let user2 = User{
    //     username:String::from("wjduser2"),
    //     email:String::from("1594803224@qq.com"),
    //     ..user1
    // }

}



