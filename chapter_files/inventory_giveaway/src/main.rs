#[derive(Debug)]
enum ShirtColor{
    Blue,
    Red,
}

struct Inventory{
    shirts: Vec<ShirtColor>
}

impl Inventory{
    fn giveaway(&self, user_preference : Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor{
        let mut red_shirts = 0;
        let mut blue_shirts = 0;

        for shirt in &self.shirts{
            match shirt{
                ShirtColor::Red => red_shirts+=1,
                ShirtColor::Blue => blue_shirts+=1
            }
        }

        if red_shirts > blue_shirts{
            ShirtColor::Red
        }else{
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory{shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]};

    let user_pref1 = Some( ShirtColor::Red );

    let giveaway1 = store.giveaway(user_pref1);

    println!("User 1 gets the shirt {:?}", giveaway1);

    let user_pref2 = None;

    let giveaway2 = store.giveaway(user_pref2);

    println!("User 2 gets the shirt {:?}", giveaway2);



}
