use crate::card::Card;

pub struct Player {
    pub hand: Vec<Card>,
    pub name: String,
}

impl Player {
	pub fn new(name: String) -> Player {
		Player {
			name: name,
			hand: Vec::<Card>::with_capacity(10),
		}
	}
}