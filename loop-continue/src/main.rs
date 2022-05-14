// * 1.continue的使用方法

fn main() {
    let mut count = 10;
    loop{
        count -= 1;
        if (count == 0) {
            // 在这里调用可以直接跳过break，循环到负数
            continue;
            break;
        }
        // 在这里调用可以跳过下面的println，不打印任何数字
        continue;
        println!("The count is {}", count);
    }
    // continue只能在loop循环体内使用
    continue;
    println!("It works");
}

