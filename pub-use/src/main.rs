mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("works");
        }
    }
}

fn main() {
    // 这里可以pub use，也可以use（how re-exporting？）
    use crate::front_of_house::hosting;
    
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }

    eat_at_restaurant();
}
