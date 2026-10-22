mod front_of_house;
use front_of_house::hosting;

mod back_of_house;
use crate::back_of_house::cooking;

fn main() {
    // crate -> front_of_house -> hosting -> add_to_waitlist
    // main.rs -> front_of_house.rs -> front_of_house/hosting.rs // Rust 2018 edition, recommended style
    hosting::add_to_waitlist();
    // crate -> back_of_house -> cooking -> prepare_meal
    // main.rs -> back_of_house/mod.rs -> back_of_house/cooking.rs // Old Style
    cooking::prepare_meal();
}
