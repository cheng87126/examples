pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    pub mod serving {
        pub fn take_order() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::serving::take_order();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}