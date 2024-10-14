use core::num;

use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self {
        // Lists
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three", "Four", "Five"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards: cards}
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, numOfCards:usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - numOfCards)
    }
}

fn main() {
    println!("Hello, world!");

    let mut deck = Deck::new();
    println!("Deck Cards : {:#?}", deck);

    deck.shuffle();
    let cards = deck.deal(3);

    println!("Deal Cards : {:#?}", cards);
    println!("Deck Cards : {:#?}", deck);
}
