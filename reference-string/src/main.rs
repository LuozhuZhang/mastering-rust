fn main() {
    // 我的确喜欢rust
    let s1 = String::from("I like rust");
    // 这样的写法会转移owner，但是&（reference）只使用value，不会转移owner
    let length = calculate_length(s1);
    println!("Which is owner? {} or {}", s1, length);
}

fn calculate_length(s: String) -> usize {
    s.len()
}
