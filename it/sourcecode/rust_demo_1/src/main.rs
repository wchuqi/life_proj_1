// 在可执行的项日中，main()函数是必须的
fn hello() {
    let a = "aa"; // Rust中的赋值，更恰当的说法叫变量绑定
    let b = "bb"; // 支持unicode编码
    let c = [a, b]; // 数组字面量使用[]
    for item in c.iter() { // iter()方法返回一个迭代器，很多类型都有iter()方法
        println!("{}", &item); // &表示借用item的值，只读访问
    }
}
fn main() {
    // println!("Hello, world!");
    // hello(); // 函数调用
    // handleCsv();

    // drop_pointer();

    // err_thread();

    // err_stackoverflow();

    // err_iter();

    int_create();
}

fn handleCsv() {
    // "\ -> 忽略掉末尾的换行符
    let data = "\
    name,length
    a,1
    b,2
    c,3
    d,4
    ";

    let rows = data.lines();

    for (i, item) in rows.enumerate() {
        // 跳过表头和空行
        if i == 0 || item.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = item
            .split(',') // 分割
            .map(|a| a.trim()) // 匿名函数，去掉字段两端空白符
            .collect(); // 构建集合

        // cfg! -> 用于编译时检查配置
        if cfg!(debug_assertions) {
            // 输出到标准错误（stderr）
            eprintln!("debug: {:?} -> {:?}", item, fields);
        }
        
        let name = fields[0];
        // 尝试着把fields[1]解析为一个32位浮点数，如果解析成功，把此浮点数赋值给变量
        if let Ok(length) = fields[1].parse::<f32>() { // 内嵌的类型注解
            println!("{}, {} cm", name, length);
        }
    }
}

// 悬垂指针例子
// 允许使用println!宏来输出枚举体Cereal
// #[derive(Debug)]
// enum Cereal { // enum(枚举体，是enumeration的缩写)，是一个具有固定数量的合法变体的类型
//     AA,BB,CC,
// }
// fn drop_pointer() {
//     // vec!也是一个宏
//     let mut grains: Vec<Cereal> = vec![]; // 初始化一个空的动态数组，其元素粪型Cereal
//     grains.push(Cereal::AA); // 动态数据里添加一个元素
//     drop(grains); // 删除动态数组
//     println!("{:?}", grains); // 试图访问已经删除的值
// }

// 数据竞争例子
// use std::thread;
// fn err_thread() {
//     let mut data = 100;
//     thread::spawn(|| { data = 200; });
//     thread::spawn(|| { data = 300; });
//     println!("{}", data);
// }

// 缓冲区溢出
// fn err_stackoverflow() {
//     let data = ['a', 'b'];
//     let val = data[3]; // error: this operation will panic at runtime
//     assert_eq!(val, 'c');
// }

// 迭代器失效
// fn err_iter() {
//     let mut data = vec!['a', 'b', 'c'];
//     for item in data {
//         println!("{}", item);
//         data.push(item.clone());
//     }
// }

// 创建整数值的多种形式
/*
要理解Rust为什么会有这么多种不同的方式，请参考以下3条原则：
1、语言的第一要务是安全性。
2、默认情况下，Rust中的数据是不可变的。
3、编译时检查是强烈推荐使用的。安全性应该是"零成本抽象"的。
*/
use std::rc::Rc;
use std::sync::{Arc, Mutex};
fn int_create() {
    let a = 10; // 在栈中的整数
    let b = Box::new(20); // 在堆中的整数
    let c = Rc::new(Box::new(30)); // 包装在一个引用计数器中的装箱的整数
    let d = Arc::new(Mutex::new(40)); // 包装在一个原子引用计数器中的整数，并由一个互斥锁保护
    println!("a:{:?}, b:{:?}, c:{:?}, d:{:?}", a, b, c, d);
}