// Crates: Modules that produce a library or executable
// Modules: Organize and handle privacy
// Packages: Build, test and share crates
// Paths: A way of naming an item such as a struct, function

mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }
    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("regular dough"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
            }
        }
    }
    pub mod help_customer {
        use core::num;

        fn seat_at_table(number: u32) {
            println!("Customer seated at table {}.", number);
        }
        pub fn take_order(number: u32, topping: &str) {
            seat_at_table(number);
            let cust_pizza: super::Pizza = super::Pizza::lunch(topping);
            serve_customer(cust_pizza);
        }
        fn serve_customer(cust_pizza: super::Pizza) {
            println!(
                "The customer is served a regular pizza with {}.",
                cust_pizza.topping
            )
        }
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order(5, "sausage");
    crate::restaurant::pizza_order::help_customer::take_order(4, "veggies");
}
