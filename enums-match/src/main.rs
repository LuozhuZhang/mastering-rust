#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 匹配到Quarter以后，必须要传入一个参数，不然会报错
        Coin::Quarter(state) => {
            // match模式匹配后，又会把这个参数给output
            println!("State quarter from {:?}!", state);
            // 而且25会被return（作为一个express），返回给传入的state变量
            25
        }
    }
}

// 调用match时，传入的参数是enmus（所以要用调用enmus中枚举成员的语法，枚举名::枚举成员）
// 在这里我们还传入了一个UsState参数
fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
}
