#![feature(generic_const_exprs)]
#![feature(array_try_from_fn)]

use combinations::{CardCombinations, num_combinations};
use highest_hand::highest_hand;
use std::{array, collections::HashMap};

pub mod combinations;
pub mod highest_hand;
pub mod io;

const FULL_DECK_SIZE: usize = 52;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
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

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
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

pub struct Results {
    wins: u64,
    draws: u64,
    losses: u64,
}

pub fn calculate<const NUM_CARDS: usize>(present_cards: [Card; NUM_CARDS]) -> Results
where
    [(); 7 - NUM_CARDS]:,
    // Seriously? FULL_DECK_SIZE is 52! We already check that 7-NUM_CARDS is valid, so 52-NUM_CARDS is as well!
    [(); FULL_DECK_SIZE - NUM_CARDS]:,
{
    let remaining_deck =
        create_deck_without_present_cards(present_cards).expect("Failed to create remaining deck");
    let player_combinations = CardCombinations::new(&remaining_deck);

    let mut player_hands: HashMap<[Card; 7 - NUM_CARDS], Hand> =
        HashMap::with_capacity(num_combinations::<
            { FULL_DECK_SIZE - NUM_CARDS },
            { 7 - NUM_CARDS },
        >());

    // Fill hashmap with player hands
    for remaining_pool in player_combinations {
        /*
        Combine the present cards with the remaining cards to create a full set of 7 cards
        I wish there was a better way to do this, but for now, this works
        Alternative:
        let combined_cards = array::from_fn(|index| {
            if index < NUM_CARDS {
                present_cards[index]
            } else {
                remaining_pool[index - NUM_CARDS]
            }
        });
        */
        let combined_cards = array_from_iter_exact(present_cards.into_iter().chain(remaining_pool))
            .expect("Failed to create combined cards");
        player_hands.insert(remaining_pool, highest_hand(combined_cards, Hand::HighCard));
    }

    Results {
        wins: todo!(),
        draws: todo!(),
        losses: todo!(),
    }
}

/// Creates a full poker deck, without the given present cards in it.
/// Returns None, if there are any duplicates in the present cards
fn create_deck_without_present_cards<const NUM_CARDS: usize>(
    present_cards: [Card; NUM_CARDS],
) -> Option<[Card; FULL_DECK_SIZE - NUM_CARDS]> {
    let iter = (2..=14)
        .flat_map(|value| (0..=3).map(move |color| (value, color)))
        .map(TryInto::try_into)
        .map(Result::unwrap)
        .filter(|card| !present_cards.contains(card));

    array_from_iter_exact(iter)
}

fn array_from_iter_exact<T, const N: usize>(mut iter: impl Iterator<Item = T>) -> Option<[T; N]> {
    let array = array::try_from_fn(|_| iter.next());

    // If the iterator isnt used up
    if iter.next().is_some() { None } else { array }
}
