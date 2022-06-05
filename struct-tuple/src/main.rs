struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // 赋值不影响
    // 但是对于函数来说，一个获取 Color 类型参数的函数不能接受 Point 作为参数
    let mut black = Color(0, 0, 0);
    let mut origin = Point(black.0, 0, 0);

    black.0 = 11;
    black.0 = origin.0;

    // 调用方法跟tuple一样，这里使用顺序
    println!("{}", origin.0);
}
