use crate::{Card, Hand};
use std::marker::PhantomData;

#[derive(Clone, Copy)]
struct SingleColored;
#[derive(Clone, Copy)]
struct MultiColored;

#[derive(Clone, Copy)]
/// A bitmap with one bit per `CardValue`
struct ValueBitmap<State> {
    // TODO: Maybe add a niche here
    inner: u16,
    _state: PhantomData<State>,
}
impl ValueBitmap<SingleColored> {
    fn is_flush(&self) -> bool {
        self.inner.count_ones() >= 5
    }
}
impl<State> ValueBitmap<State> {
    /// Returns the iteration the straight was found on, if any
    fn is_straight(&self) -> Option<u8> {
        let straight_mask = 0b0001111100000000;
        for shift in 0..9 {
            let mask = straight_mask >> shift;
            if self.inner & mask == mask {
                return Some(shift);
            }
        }
        None
    }
    fn new(inner: u16) -> Self {
        Self {
            inner,
            _state: PhantomData,
        }
    }
}

/// One `ValueBitmap` per color
struct ColorValueBitmaps([ValueBitmap<SingleColored>; 4]);
impl From<[Card; 7]> for ColorValueBitmaps {
    fn from(cards: [Card; 7]) -> Self {
        let mut color_value_bitmaps = ColorValueBitmaps([ValueBitmap::new(0); 4]);

        for card in cards {
            color_value_bitmaps.0[card.color as usize].inner |= 1 << card.value as u8;
        }

        color_value_bitmaps
    }
}
impl ColorValueBitmaps {
    fn get_flush(&self) -> Option<ValueBitmap<SingleColored>> {
        self.0
            .into_iter()
            .find(|value_bitmap| value_bitmap.is_flush())
    }
    fn is_four_of_a_kind(&self) -> bool {
        // TODO: See if manually initialising with first.inner is faster
        self.0
            .into_iter()
            // AND all values together
            .fold(u16::MAX, |acc, value_bitmap| acc & value_bitmap.inner)
            != 0
    }
}

pub fn highest_hand(cards: [Card; 7]) -> Hand {
    let color_value_bitmaps = ColorValueBitmaps::from(cards);

    // Check for Royal & Straight flushes
    if let Some(flush) = color_value_bitmaps.get_flush()
        && let Some(iter) = flush.is_straight()
    {
        return if iter == 0 {
            Hand::RoyalFlush
        } else {
            Hand::StraightFlush
        };
    }

    if color_value_bitmaps.is_four_of_a_kind() {
        return Hand::FourOfAKind;
    }

    todo!()
}
