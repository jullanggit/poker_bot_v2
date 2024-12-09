#![feature(generic_const_exprs)]
#![feature(array_try_from_fn)]

use std::{array, collections::HashMap};

pub mod combinations;
pub mod io;

const FULL_DECK_SIZE: usize = 52;

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
impl TryFrom<(u8, u8)> for Card {
    type Error = &'static str;

    fn try_from((value, color): (u8, u8)) -> Result<Self, Self::Error> {
        Ok(Self::new(value.try_into()?, color.try_into()?))
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
impl From<CardValue> for u8 {
    fn from(value: CardValue) -> Self {
        match value {
            CardValue::Two => 2,
            CardValue::Three => 3,
            CardValue::Four => 4,
            CardValue::Five => 5,
            CardValue::Six => 6,
            CardValue::Seven => 7,
            CardValue::Eight => 8,
            CardValue::Nine => 9,
            CardValue::Ten => 10,
            CardValue::Jack => 11,
            CardValue::Queen => 12,
            CardValue::King => 13,
            CardValue::Ace => 14,
        }
    }
}
impl TryFrom<u8> for CardValue {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(CardValue::Two),
            3 => Ok(CardValue::Three),
            4 => Ok(CardValue::Four),
            5 => Ok(CardValue::Five),
            6 => Ok(CardValue::Six),
            7 => Ok(CardValue::Seven),
            8 => Ok(CardValue::Eight),
            9 => Ok(CardValue::Nine),
            10 => Ok(CardValue::Ten),
            11 => Ok(CardValue::Jack),
            12 => Ok(CardValue::Queen),
            13 => Ok(CardValue::King),
            14 => Ok(CardValue::Ace),
            _ => Err("Invalid card value"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl From<Color> for u8 {
    fn from(value: Color) -> Self {
        match value {
            Color::Hearts => 1,
            Color::Diamonds => 2,
            Color::Clubs => 3,
            Color::Spades => 4,
        }
    }
}
impl TryFrom<u8> for Color {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Color::Hearts),
            2 => Ok(Color::Diamonds),
            3 => Ok(Color::Clubs),
            4 => Ok(Color::Spades),
            _ => Err("Invalid color"),
        }
    }
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

fn calculate<const NUM_CARDS: usize>(present_cards: &[Card; NUM_CARDS]) -> Results
where
    [(); 7 - NUM_CARDS]:,
{
    let (player_cards, present_pool) = present_cards.split_at(2);

    let player_hands: HashMap<[Card; 7 - NUM_CARDS], Hand> = HashMap::new();
}

