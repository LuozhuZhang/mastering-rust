// statement执行某个操作，但不会返回数据
// expression执行计算，并返回数据（所以在有返回数据的function中要注意，这也是rust和其他语言的区别之一）
fn main() {
    let y = {
        let x = 3;
        // 不能带分号，否则作为statement无法返回值，无法给y赋值
        x + 1;
    };

    println!("The number is {}", y);
}

// rust中必须指定function返回的数据类型
fn number() -> i32{
    // 无return，则默认返回最后一个expression
    3
}