use std::time::Duration;

fn main() {

 let mut num = Vec::with_capacity(11);

 {
    std::thread::spawn(move || {
        for i in 0..10 {
           num.push(i);
        }
    });
 }

 {
    println!("{}", num.capacity());
 }
}

// fix thread issue