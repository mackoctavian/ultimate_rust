use std::{ time::Duration};

fn main() {
    let store = Invetory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user1_preference = ShirtColor::Blue;
    let giveaway1 = store.giveaway(Some(user1_preference));
    println!("The user with preference {:?} gets {:?}", user1_preference, giveaway1);

    //Closures with type annotation
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slow...");
        std::thread::sleep(Duration::from_secs(2));
        num
    };

    expensive_closure(2);
}

#[derive(Debug, Clone,Copy)]
enum ShirtColor {
    Red,
    Blue
}

#[derive(Debug, Clone)]
struct Invetory {
    shirts: Vec<ShirtColor>,
}

impl Invetory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_blue > num_red {
            return  ShirtColor::Blue;
        }

        ShirtColor::Red
    }
}