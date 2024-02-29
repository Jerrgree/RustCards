pub mod card;    
mod deck;
use deck::Deck;


fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let card = deck.draw();

    match card {
        Some(x) => println!("{x}"),
        None => println!("No Cards Remaining"),
    }
}