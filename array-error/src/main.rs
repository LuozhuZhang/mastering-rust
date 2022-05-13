// Input and Output：从键盘中读取输入并打印出来
// More info：https://doc.rust-lang.org/stable/std/io/fn.stdin.html
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Ready, pls input a index!");

    // 继承String的属性，创建一个新的variable，因为键盘输入的数字是string（后面需要转成uint）
    let mut index = String::new();

    // io::stdin()从输入流中读取数据，read_line存储输入流中的数据（输入的数据存在index）
    io::stdin().read_line(&mut index).expect("Failed to read the line");

    // 对字符串做处理，trim消除所有的whitespace，parse把数据解析为其他类型
    let index:usize = index.trim().parse().expect("It's not a number");

    let element = a[index];

    // output
    println!(
        "Input the number {}, output the number {}",
        index, element
    );
}
