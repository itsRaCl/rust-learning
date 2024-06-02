mod front_of_house {

    pub mod hosting {
        pub mod t_o {


            pub fn add_to_waitlist() {
            }
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        pub fn serve_order() {
            take_order();
            take_payment();
        }

        fn take_payment() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::t_o::add_to_waitlist();
}
