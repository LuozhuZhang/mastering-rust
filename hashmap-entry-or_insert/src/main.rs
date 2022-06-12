fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // 1）split_whitespace可以根据空格分词
    // 这里把hello、world、wonderful、world等词分别打印了出来
    for word in text.split_whitespace() {
        // 2）count可以计算一个word出现了几次
        // 第一次出现就插入word作为key，并插入0作为value
        // entry会返回插入值，在这里是0
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

// * 3）entry的实际定义如下：
// #[inline]
// #[stable(feature = "rust1", since = "1.0.0")]
// pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
//     map_entry(self.base.rustc_entry(key))
// }
