fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    // 把s值给删了，看看编译器会不会报错
    s.clear();

    println!("s content is {}", word);
}

// &str返回静态类型（不可变引用，指向binary文件），string返回动态类型
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
        // println!("this {}", i);
        // println!("this {}", item);
    }

    // s.len() // 数字要用usize
    &s[..] // 返回整个字符串切片
}


// set中也有一个slice，后面再看