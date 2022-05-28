fn main() {
    let s = String::from("hello world");
    // String to Bytes
    let bytes = s.as_bytes();
    // Get byte from bytes array
    // ? How to print bytes array
    for i in bytes {
        println!("This is {}", i);
    }
}

// Create a new  programming language with Rust：https://createlang.rs/intro.html
// Build a interpreter with Rust：https://rust-hosted-langs.github.io/book/
