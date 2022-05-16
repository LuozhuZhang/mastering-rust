// * reference的可变问题

fn main() {
    // * 常出现的错误，如果declare的variable不是mutable，就算用&mut也无法改变value
    // let s1 = String::from("i like rust");
    let mut s1 = String::from("i like rust");
    change_reference(&mut s1);
    
}

// * reference默认是不可变的，不难理解，因为reference只不过是指向value指针的pointer而已，不拥有value，无法通过这种方式改变value
// fn change_reference(s: &String) {
//     s.push_str("ok fine");
// }

// * 需要改变reference的value就需要用到大名鼎鼎的&mut了（直接通过owner改也可以）
fn change_reference(s: &mut String) {
    s.push_str("ok fine");
}