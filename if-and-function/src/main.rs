// 1.写一段程序，if true返回数字，else返回字符串（不同类型怎么处理）
// 2.函数如果要返回字符串怎么定义？

// * 1.条件控制，并返回不同类型
fn main() {
    let number = 6;
    // rust中if-else返回的数据类型都必须相同
    // 其中一个方案是改用match：https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html
    if number == 3 {
        println!("Output String");
    } else {
        println!("number is divisible by 2");
    }
}

// * 2.函数返回字符串
// return type不能定义为&str（具体原因 之后研究），需要定义为String
fn fc() -> String{
    // 需要对字符串做一个处理：change &str to String
    let s: String = "asdasdasd".to_string();
    s
    // 下面这样也可以
    "asdasdasd".to_string()
}


