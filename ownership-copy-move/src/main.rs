// * 这样的代码是copy，也就是把x1的值拷贝到x2上，创建了两个variable，stack上存储了两个数据
fn main() {
    let x1 = 3;
    let x2 = x1;
    // 你会发现都可以输出
    println!("{} , {}", x1, x2);

    // * 这样的代码是move，数据还在heap/stack上，s2和s1都指向了数据的同一位置，但是为了防止double free的问题（s1和s2都指向同一个value，如果s1离开scope，其指向的value会被release，s2没有离开scope，但是也失去了value）
    // rust为了解决这个问题（也是因为rust独特的ownership才会有这个问题），将两个owner指向同一个的情况定位move（之前的owner会消失，所以我们会看到下面的s1报错）
    let s1 = String::from("Hi");
    let s2 = s1;
    println!("{} , {}", s1, s2);

    // * 深度clone数据，不仅是stack，还有heap
    // 但是如果数据量大，并且heap操作本来就比较复杂/耗时，可能比较消耗性能
    let k1 = String::from("Hi");
    let k2 = k1.clone();
    println!("{} , {}", k1, k2);
}
// 别忘了ownership rule：value在任一时刻，有且只有一个owner