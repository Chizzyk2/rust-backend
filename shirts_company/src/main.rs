
mod shirt;
use crate::shirt::{Inventory, ShirtColor};

fn main(){
    let store = Inventory{
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_prefer1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_prefer1);
    println!("The user with preference {:?} gets {:?}", user_prefer1, giveaway1);

    let user_prefer2 = None;
    let giveaway2 = store.giveaway(user_prefer2);
    println!("The user with preference {:?} gets {:?}",user_prefer2, giveaway2);
}