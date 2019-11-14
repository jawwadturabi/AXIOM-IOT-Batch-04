// mod front_of_house {
//     fn my() {}
// add_to_waitlist()
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}
//     add_to_waitlist()
//         fn serve_order() {}
//         fn take_payment() {}
//     }

//     // #[derive(Debug)]
//     // struct Name {
//     //     field: Type,
//     // }
// }

mod front_of_house {
    fn my() {}
    pub mod hosting {
        pub fn add_to_waitlist() {
            super::my()
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
