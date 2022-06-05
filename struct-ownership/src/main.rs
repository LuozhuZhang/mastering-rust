struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("someone@example.com"),
        // 使用结构更新语法会造成的影响
        ..user1
    };

    // 这里相当于赋值语句（=），user1.username的owner发生了转移
    // 转移给user2之后，就不能使用user1的value了
    println!("{}", user1.username);
    println!("{}", user2.username);
}

