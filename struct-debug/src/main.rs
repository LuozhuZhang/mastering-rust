// 这是一个外部属性，打开rust的debug模式
// 而且debug是一个trait，这是个第十章才了解到的概念
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // struct中不能直接输出实例对象（rect1），但是可以输出结构体中的值（比如rect1.width）
    // 根本原因是因为struct中没有实现Display（用于直接output展示输出）
    println!("rect1 is {:?}", rect1);
}

// {:?} 和 {:#?} 会输出不同的内容，前者只输出一行，后者按照结构体格式输出