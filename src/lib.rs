#![feature(generic_const_exprs)]
#![feature(array_try_from_fn)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]

use combinations::{CombinationMap, Combinations, num_combinations};
use highest_hand::highest_hand;
use std::{array, mem::MaybeUninit, ptr};

pub mod combinations;
pub mod highest_hand;
pub mod io;

const FULL_DECK_SIZE: usize = 52;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
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
    #[default]
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

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub enum Color {
    #[default]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

#[derive(Default, Debug)]
pub struct Results {
    wins: u64,
    draws: u64,
    losses: u64,
}

fn combine_cards_with_indices<const N: usize, const DECK_SIZE: usize>(
    cards: &[Card],
    indices: [usize; N],
    deck: &[Card; DECK_SIZE],
) -> [Card; 7] {
    debug_assert!(cards.len() + indices.len() == 7);

    let mut combined_cards = MaybeUninit::uninit_array();

    // SAFETY
    // using copy_nonoverlapping is fine, because we copy into a locally defined array
    unsafe {
        ptr::copy_nonoverlapping(
            cards.as_ptr() as *const MaybeUninit<Card>,
            combined_cards.as_mut_ptr(),
            7 - N,
        );
    }

    for (cards_index, deck_index) in indices.into_iter().enumerate() {
        combined_cards[cards_index + 7 - N] = MaybeUninit::new(deck[deck_index]);
    }

    unsafe { MaybeUninit::array_assume_init(combined_cards) }
}

pub fn calculate<const NUM_CARDS: usize>(present_cards: [Card; NUM_CARDS]) -> Results
where
    [(); 7 - NUM_CARDS]:,
    // Seriously? FULL_DECK_SIZE is 52! We already check that 7-NUM_CARDS is valid, so 52-NUM_CARDS is as well!
    [(); FULL_DECK_SIZE - NUM_CARDS]:,
    [(); 9 - NUM_CARDS]:,
    // N = FULL_DECK_SIZE - NUM_CARDS, R = 7 - NUM_CARDS
    [(); num_combinations(FULL_DECK_SIZE - NUM_CARDS, 7 - NUM_CARDS)]:,
    [(); FULL_DECK_SIZE - NUM_CARDS - 1]:,
    [(); 7 - NUM_CARDS - 1]:,
{
    let remaining_deck =
        create_deck_without_present_cards(present_cards).expect("Failed to create remaining deck");

    let mut player_hands: CombinationMap<{ FULL_DECK_SIZE - NUM_CARDS }, { 7 - NUM_CARDS }> =
        const { CombinationMap::new() };

    // Fill hashmap with player hands
    for (i, remaining_pool_indices) in
        Combinations::<{ FULL_DECK_SIZE - NUM_CARDS }, { 7 - NUM_CARDS }>::new().enumerate()
    {
        let combined_cards =
            combine_cards_with_indices(&present_cards, remaining_pool_indices, &remaining_deck);

        // This iterator should be in lexicographic order, so directly indexing the array should be fine
        player_hands.array[i] = highest_hand(combined_cards);
    }

    let mut results = Results::default();

    // Calculate results
    // For all possible remaining cards
    for card_indices in Combinations::<{ FULL_DECK_SIZE - NUM_CARDS }, { 9 - NUM_CARDS }>::new() {
        let combined_cards =
            combine_cards_with_indices(&present_cards[2..], card_indices, &remaining_deck);
        let highest_hand = highest_hand(combined_cards);

        for remaining_pool in
            Combinations::<{ FULL_DECK_SIZE - NUM_CARDS }, { 7 - NUM_CARDS }>::new()
        {
            let player_hand = player_hands[remaining_pool];

            match highest_hand.cmp(&player_hand) {
                std::cmp::Ordering::Less => results.losses += 1,
                std::cmp::Ordering::Equal => results.draws += 1,
                std::cmp::Ordering::Greater => results.wins += 1,
            }
        }
    }

    results
}

// Necessary because caluclate doesnt work with seven cards
pub fn calculate_7(present_cards: [Card; 7]) -> Results {
    let remaining_deck =
        create_deck_without_present_cards(present_cards).expect("Failed to create remaining deck");

    let player_hand = highest_hand(present_cards);

    let mut results = Results::default();

    // Calculate results
    // For all possible remaining cards
    for card_indices in Combinations::<{ FULL_DECK_SIZE - 7 }, 2>::new() {
        let combined_cards =
            combine_cards_with_indices(&present_cards[2..], card_indices, &remaining_deck);
        let highest_hand = highest_hand(combined_cards);

        match highest_hand.cmp(&player_hand) {
            std::cmp::Ordering::Less => results.losses += 1,
            std::cmp::Ordering::Equal => results.draws += 1,
            std::cmp::Ordering::Greater => results.wins += 1,
        }
    }
    results
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

#[cfg(test)]
mod tests {
    use crate::{Card, CardValue, Color, combine_cards_with_indices};

    #[test]
    fn test_combine_cards_with_indices() {
        let deck = [Card::default()];

        let cards = [Card::new(CardValue::Ace, Color::Clubs); 5];
        let indices = [0; 2];

        let combined = combine_cards_with_indices(&cards, indices, &deck);

        assert_eq!(combined, [
            Card::new(CardValue::Ace, Color::Clubs),
            Card::new(CardValue::Ace, Color::Clubs),
            Card::new(CardValue::Ace, Color::Clubs),
            Card::new(CardValue::Ace, Color::Clubs),
            Card::new(CardValue::Ace, Color::Clubs),
            Card::default(),
            Card::default(),
        ]);
    }
}
