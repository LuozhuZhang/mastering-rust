fn main() {
    let mut s = String::from("ok");

    let mut s1 = &mut s;
    // rust中一个value不能同时被两个&mut reference，可以理解为value在任何时刻尤其仅有一个owner
    // 但如果超出了ownership，就没什么问题了（因为之前的reference将被释放）
    let s2 = &mut s;
    // 对&mut reference再进行一次reference是可以的
    let s2 = &mut s1;
}

