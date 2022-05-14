// * 对于heap，分配（allocating）可变数据的时候分为两步
// 1）运行时向memory allocator请求内存
// 2）计算后将内存返回给allocator

fn main() {
    // "xxxx"默认是&str格式，就算添加了mut也是不可变的数据，这类数据存储到heap上，因为compile的时候没办法知道确切大小（比如用户输入时）
    // let mut s = "mother";

    // * 在调用String::from的时候，就是向memory allocator请求一个内存
    // 用完内存以后，GC（garbage collection）语言会自动释放内存；C/C++这些无GC语言可能需要手动显式释放内存；而rust使用ownership，代码超出scope之后自动释放内存（为每个allocate分配一个free）
    let mut s = String::from("mother");
    s.push_str(" fuck");
    println!("{}", s);
}   // 如果s离开了scope，value就会被丢弃，释放内存 还给allocator
// 除了这个scope就调用drop（rust的独特实现）