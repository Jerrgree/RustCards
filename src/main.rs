pub mod card;    
mod deck;
use deck::Deck;


fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    while let Some(card) = deck.draw() {
        println!("{card}");
    }
}