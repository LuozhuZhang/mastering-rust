// dbg!非常好用，由此可见dbg!还是一个debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // 如果是express，dbg!计算express，并输出最终值
        width: dbg!(30 * scale),
        height: 50,
    };

    // 输出的格式类似于 {:#?}
    dbg!(rect1);

    // 一个重点：
    // dbg!会改变ownership，如果不想让dbg!拥有结构体的值，不要忘了使用reference
    // 在这里没有使用reference，所以结构体value的owner发生了转变（转移给了dbg!），我们无法使用rect1的值
    println!("{:?}", rect1);
}

// trait很神奇也很有意思，在rust中我们后面也可以实现自己的trait