use crate::card::Card;

pub struct Player<'a> {
    pub hand: Vec<Card>,
    pub name: &'a str,
}

impl Player<'_> {
	pub fn new(name: &str) -> Player {
		Player {
			name: name,
			hand: Vec::<Card>::with_capacity(10),
		}
	}
}