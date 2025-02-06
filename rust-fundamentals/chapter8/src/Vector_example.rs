fn main() {
    //创建Vector
    let v: Vec<i32> = Vec::new(); //Vec::new()创建的是一个空的所有需要在声明变量的时候指名变量的参数类型
                                  //使用初始值创建Vec<T>,使用vec!宏
    let mut v1 = vec![1, 2, 3];

    // 更新Vector使用的是push方法
    v1.push(4);
    v1.push(4);
    v1.push(6);

    // 读取Vector的元素，一种是使用索引的方式，二是使用get方法
    //方法1：直接索引访问，适用于确定索引有效的情况，但会导致 panic 如果索引无效
    let third: &i32 = &v1[3];
    println!("取到的Vector的第四个元素值为：{}", third);

    //方法2：使用 get 方法进行安全访问，适用于不确定索引是否有效的情况，避免了 panic。根据你的需求和场景，选择合适的方式来访问 Vector 的元素。
    match v1.get(3) {
        Some(third) => println!("取到的Vector的第四个元素值为：{}", third),
        None => println!("没有获取到相关Vector数值"),
    }

    // 所有权和借用规则，不能在同一作用域内同时拥有可变和不可变引用
    let mut v2 = vec![1, 2, 3, 4, 5, 6];
    let first = &v2[0];
    // v2.push(9);
    println!("这是第一个元素{}", first);

    //便利Vector当中的所有值
    for i in &mut v2 {
        println!("Vector v2 当中的所有遍历的值为（修改前）：{}", i);
    }
    for i in &mut v2 {
        *i += 50; //对i进行解引用
    }
    for i in &mut v2 {
        println!("Vector v2 当中的所有遍历的值为（修改后）：{}", i);
    }

    //删除Vector，当Vector离开作用域后它就被清理掉了，所有的元素也被清理掉
}
