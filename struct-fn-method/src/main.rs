#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 代码变得非常有组织性，一类struct的method操作放到一个impl块中就可以了
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// 迫不及待用rust实现更多好玩的有趣项目

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}