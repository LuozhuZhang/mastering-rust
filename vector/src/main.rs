fn main() {
    // let mut v = vec![1, 2, 3, 4, 5];

    // * &v可以refer整个vector的数据，如何用{:?}可以导出所有数据
    // let v = &v;

    // println!{"{:?}", v};


    // * 不能在一个scope中同时存在可变引用和不可变引用
    let mut v = vec![1, 2, 3, 4, 5];

    // 1）这里是一个不可变引用
    let first = &v[0];
    // 2）如果这里对原数据作了修改（这里被rust理解为可变引用？）
    // 后面的reference也失效了，不能调用first的值
    v.push(6);

    println!("The first element is: {}", first);

}

