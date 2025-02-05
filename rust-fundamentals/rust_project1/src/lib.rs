mod front_of_house{
    pub mod hosting{
       pub fn add_to_waitlist(){

        }
    }
}
//在rust当中所有的函数、变量、struct、enum等都是私有的，要想变成公共的需要添加pub
pub fn eat_at_restaurant(){
    crate:: front_of_house::hosting::add_to_waitlist();//使用绝对路径形式来调用函数
    front_of_house::hosting::add_to_waitlist();//使用相对路径形式来调用函数
}

//在rust语言中使用super关键字来访问父级模块路径中的内容，类似文件系统中的..
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();//使用的是相对路径
        crate::serve_order();//使用的是绝对路径
    }

    fn cook_order() {}
}
//结构体可设置为公有的，pub放在struct之前，struct是公共的，struct的字段默认是私有的，struct的字段需要单独设置pub来变成共有的
mod back_of_house{
    pub struct Breakfast{
        pub toast:String,//添加pub内部就是公共的，不添加是私有的

        seasonal_fruit:String,
    }
}