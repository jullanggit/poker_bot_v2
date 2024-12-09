#![feature(array_try_from_fn)]

pub mod combinations;
pub mod io;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Card {
    value: CardValue,
    color: Color,
}
impl Card {
    pub fn new(value: CardValue, color: Color) -> Self {
        Self { value, color }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum Hand {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

struct Results {
    wins: u64,
    draws: u64,
    losses: u64,
}

fn calculate<const NUM_CARDS: usize>(cards: &[Card; NUM_CARDS]) -> Results {
    let (player_cards, present_pool) = cards.split_at(2);
}
