use std::time::Duration;

fn main() {

 let mut num = Vec::with_capacity(11);

 let a = {
    let a = std::thread::spawn(move || {
        for i in 0..10 {
           num.push(i);
        }
        num
    });
    a
 };

 let b = a.join().unwrap();
 {
    println!("{}", b.capacity());
 }
}

// fix thread issue
