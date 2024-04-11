use crate::front_of_house::hosting;
// or self::front_of_house::hosting


// called by front_of_house::serving::use_serve_order
fn serve_order() {}

// find module in the same name file
mod seperate_file;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        // use of `super` means calling the parent of this mod
        fn use_serve_order() {
            super::super::serve_order();
        }

        fn take_payment() {}
    }
}

// paths
pub fn eat_at_restaurant() {
    // absolute path
    // `crate` is the root dir of lib.rs
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    //`use`
    hosting::add_to_waitlist();
}