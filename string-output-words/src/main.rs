// * 一段简单的程序，输入一个字符串，返回第一个单词（通过空格检测）
// * 空格的byte是32

fn main() {
    let mut s = String::from("hello world");

    // * return string的长度是5，为什么把5保存到word之后，s的内容就发生改变了？
    let word = first_word(&s); // word 的值为 5
    println!("s content is {}", s);

    s.clear(); // 错误! word引用的值没了
    println!("s content is {}", s);

    // * 没有使用slice的时候，就算word引用的值s发生改变（clear了），word也可以使用（这是一个大bug）
    // * 使用slice可以弹出error，避免我们遇到这种问题
    println!("works? {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter()迭代，在这里跟 for i 一起构成 loop
    for (i, &item) in bytes.iter().enumerate() {
        // * 检查是否有空格，发现空格则返回它的位置 i
        if item == b' ' {
            return i;
        }
        // * 事实证明，i是bytes（一个byte array）的索引，&item是数组中值的reference。这里使用的应该是一个叫“模式”的东西，在第六章，之后仔细看看
        println!("this {}", i);
        println!("this {}", item);
        // * 另外这里只返回一个word，所以i和item都只有 h、e、l、l、o，五个字母
    }

    s.len()
}