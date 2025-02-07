#![allow(unused)]
use std::{collections::HashMap, string};
use unicode_segmentation::UnicodeSegmentation;
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
    println!("级联多个字符串{}", a4);

    //对于更为复杂的字符串，可以使用format!宏
    let b1 = String::from("tic");
    let b2 = String::from("tac");
    let b3 = String::from("toe");

    let b4 = format!("{}-{}-{}", b1, b2, b3);
    println!("使用format!宏{}", b4);

    //索引字符串,在rust语言当中无法通过引用字符串中的单独字符来进行索引
    //索引操作消耗一个常量（O(1)）,而String无法保证：需要便利所有内容，来确定有多少个合法的字符
    // let b4_index = b4[0];
    //使用Unicode标量值来表示字母，每个Unicode标量值都对应两个字节，字节索引并不能总是有效的对应一个标量值
    //rust当中包含三种看待字符串的形式：字节、标量值、字形簇
    let sanskrit = "नमस्ते";
    //打印字节形式
    for a in sanskrit.bytes() {
        println!("{}", a);
    }
    //使用Unicode标量值形式返回
    for b in sanskrit.chars() {
        println!("{}", b);
    }
    //使用字形簇的方式来进行返回，返回值更接近真实字母
    for c in sanskrit.graphemes(true) {
        println!("{}", c);
    }

    //字符串 slice
    let hello = "Здравствуйте";

    let pri_s = &hello[0..4]; //当索引设置为[0..3]是程序会出现恐慌因为3不是字符串的边界
                              //切割字符串时允许的但是必须谨慎使用，如果切割时跨越字符串，程序就会出现恐慌panic
    println!("{}", pri_s);

    println!("*******************************************************************");
    println!("哈希map！！！");
    // HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中。通过K（任何类型）来寻找数据，而不是通过索引。
    //Hashmap用的较少，不在Prelude中，标准库对其支持较少，没有内置的宏来创建Hashmap
    //数据存储在heap上
    //Hashmap是同构的：一个Hashmap中所有的K必须是同一种类型、所有的V必须是同一种类型
    let mut scores = HashMap::new(); //定义HashMap时需要指明K和V的值
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    //另外一种创建Hashmap的方法：collect
    //在元素类型为Tuple的Vector上使用collect方法，组建一个Hashmap：要求Tuple有两个值一个作为K，一个作为V，collect可以吧数据整合成多种数据类型其中包括Hashmap
    let teams = vec![String::from("blue"), String::from("red")];
    let intial_scores = vec![10, 20];

    //iter() 方法用于创建向量的迭代器。teams.iter() 返回一个迭代器，该迭代器按顺序产生 teams 向量中每个元素的不可变引用.
    //zip 方法用于将两个迭代器组合成一个新的迭代器，该新迭代器产生的元素是由两个原始迭代器对应位置的元素组成的元组。
    //collect 方法用于将迭代器中的元素收集到一个集合中。需要指明返回类型的变量HashMap<_,_>
    let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();

    // Hashmap和所有权
    // 对于实现了copy trait的类型（i32），值会被复制到Hashmap中，对于拥有所有权的值（例如String），值会被移动，所有权会转移给Hashmap
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！

    // 访问哈希 map 中的值,访问哈希 map 中的值
    let mut hashmapget = HashMap::new();
    hashmapget.insert(String::from("wjd"), 10);
    hashmapget.insert(String::from("test"), 20);

    let name = String::from("wjd");
    let find_hashmapget=hashmapget.get(&name);//get方法返回的是一个option枚举
    match find_hashmapget{
        Some(s)=>println!("{}",s),
        None =>println!("is not exist"),
    }

    // 遍历Hashmap
    for (k,v) in &hashmapget{
        println!("k:{},v:{}",k,v);
    }


    //更新Hashmap
    //Hashmap大小可变、每个K同时只能对应一个V
    //更新Hashmap当中的数据：（1）K已经存在，对应一个V。a.替换现有的V。b.保留现有的V，忽略新的V。c.合并现有的V和新的V。（2）K不存在，添加一对新的K,V

    // 替换现有的 V,如果向 HashMap 插入一对 KV，然后再插入同样的K，但是不同的 V，那么原来的V会被替换掉
    let mut hashTableInsertion = HashMap::new();
    hashTableInsertion.insert(String::from("blue"), 10);
    hashTableInsertion.insert(String::from("blue"), 25);
    println!("{:?}",hashTableInsertion);

    //只在K不对应任何值的情况下才插入V。使用entry方法能够检查指定的K是否对应一个V，参数为K，返回值为enum Entry代表值是否存在
    let newColor = hashTableInsertion.entry(String::from("yellow"));
    newColor.or_insert(50);//当V值不存在的情况下才进行插入
    hashTableInsertion.entry(String::from("blue")).or_insert(50);
    println!("{:?}",hashTableInsertion);









}
