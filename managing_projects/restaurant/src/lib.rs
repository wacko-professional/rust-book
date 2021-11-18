// Modules are useful to  organize your code and define Rust's privacy boundary
// If you want to make an item like a function or struct private, you put it in a module
// Items in a parent module cant use private items inside child modules, but items in child
// modules can use items in their ancestor modules
// BUT you can expose inner parts of child modules' code to outer ancestor modules by using pub
// keyword to make an item public
mod front_of_house {
    // Note: Making the module public doesnt make its contents public
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//----------------------
//fn serve_order() {}
//
//mod back_of_house {
//    fn fix_incorrect_order() {
//        cook_order();
//        // the fix_incorrect_order function is in the back_of_house module
//        // super refers to the parent module of back_of_house (in this case, is crate)
//        // makes sense to use super here if we think that the back_of_house module and the
//        // serve_order function are likely to stay in the same relationship to each other
//        // and get moved together should we decide to reorganize the crate's module tree
//        // fewer places to update code
//        super::serve_order();
//    }
//    fn cook_order() {}
//}
//-----------------------

// We can use pub to designate structs and enums as public
// But, if we use pub before a struct definition, we make the struct public but the fields will
// still be private
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // if we make an enum public, all of its variants are then public
    pub enum Appetizer {
        Soup,
        Salad
    }

    impl Breakfast {
        // Note: We need to provide a public associated function that constructs an instance of
        // Breakfast because seasonal_fruit is private
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // We can write and read to the toast field using dot notation because the toast field
    // in the back_of_house::Breakfast struct is public
    meal.toast = String::from("Wheat");
    // Cannot do this because seasonal_fruit is private
    //meal.seasonal_fruit = String::from("blueberries");
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

//// new function (mark it with the pub  keyword  as the function is part of our library crate's
//// public API)
//// eat_at_restaurant and front_of_house are siblings
//pub fn eat_at_restaurant() {
//    // Absolute path (the add_to_waitlist function is defined in the SAME CRATE as
//    // eat_at_restaurant, which means we can use the crate keyword to start an abs path)
//    crate::front_of_house::hosting::add_to_waitlist();
//
//    // Relative path
//    // starting from the name of the module defined at the same level of the module tree
//    // starting with a name means that the path is relative
//    front_of_house::hosting::add_to_waitlist();
//}

//// Define a module by starting with the mod keyword, then specify the name of the module
//// in this case, name is front_of_house
//mod front_of_house {
//    // Inside modules, we can have other modules
//    mod hosting {
//        // Modules can also hold definitions for other items (such as functions)
//        fn add_to_waitlist() {}
//
//        fn seat_at_table() {}
//    }
//
//    // Using modules allow us to group related definitions together and name why they're related
//    // Programmers adding new functionality would know where to place the code to keep the
//    // program organized
//    mod serving {
//        fn take_order() {}
//
//        fn serve_order() {}
//
//        fn take_payment() {}
//    }
//}
