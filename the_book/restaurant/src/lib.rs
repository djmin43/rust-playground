mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

use crate::front_of_house::hosting;

use crate::back_of_house::kitchen;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();


    kitchen::washing_dishes();

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {

    pub mod kitchen {
        pub fn washing_dishes() {

        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}