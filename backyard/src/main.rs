use crate::garden::vegetables::Apsaragus;

pub mod garden;

fn main() {
    let plant = Apsaragus {};
    println!("I'm planting {:?}!", plant);
}
