use hello_module::customer::eat_at_restaurant;
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);

    let customer = eat_at_restaurant();
}
