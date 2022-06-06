fn main() {
    // Some背后是Option<T>的枚举实现
    let config_max = Some(3u8);
    // if let语法（本质上是一个语法糖），跟 _ => 的作用差不多
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // 字符串做模式无法work
    let try_string = String::from("love rust");
    if let String::from("love rust") = try_string {
        println!("The maximum is configured to be",);
    }
}
