// * 斐波那契数列：当n=1或n=2时F(n)=1，F(n)=F(n-1)+F(n-2)
// wikipedia：https://zh.wikipedia.org/zh-tw/%E6%96%90%E6%B3%A2%E9%82%A3%E5%A5%91%E6%95%B0

fn main() {
    // 构建一个斐波那契：0、1、1、2、3、5、8 ... 
    println!("Fib Sequence is {}", fib(6));
}

fn fib(n: i32) -> i32 {
    // 数列索引必须大于0
    if n <= 0 {
        println!("no zero!");
    }
    // 用递归的方式来实现，也可以用刚刚学到的loop来实现
    if n<=2 { 1 } else { fib(n-1) + fib(n-2) }
}