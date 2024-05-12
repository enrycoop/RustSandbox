// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("I'm in the waitlist..");
//             seat_at_table();
//         }
//         // simbling
//         fn seat_at_table() {
//             println!("I'm sitting on the table.");
//         } 
//     }
// }

// pub fn eat_at_resturant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();

    
// }


// Super

// fn deliver_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }

// pub to enums and structs
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_resturant() {
    // Ordina una colazione con un rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Cambiamo idea riguardo al pane
    meal.toast = String::from("Wheat");
    println!("Vorrei un {} toast per favore", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    //meal.seasonal_fruit = String::from("blueberries");

    // a differenza di enum public, tutte le sue varianti sono public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

