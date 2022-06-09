fn main() {
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }
    
        impl Breakfast {
            // 返回一个public的实例
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }
    
    pub fn eat_at_restaurant() {
        // 这里只能调用summer实例（一个关联函数）
        // 因为有private的字段
        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
    
        // meal.seasonal_fruit = String::from("blueberries");
    }
}
