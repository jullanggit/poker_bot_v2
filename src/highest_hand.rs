use crate::{Card, Hand};
use std::marker::PhantomData;

#[derive(Clone, Copy)]
struct SingleColored;
#[derive(Clone, Copy)]
struct MultiColored;

#[derive(Clone, Copy)]
/// A bitmap with one bit per `CardValue`
struct ValueBitmap<State> {
    inner: u16,
    _state: PhantomData<State>,
}
impl ValueBitmap<SingleColored> {
    fn is_flush(&self) -> bool {
        self.inner.count_ones() >= 5
    }
}
impl<State> ValueBitmap<State> {
    fn is_straight(&self) -> bool {
        let straight_mask = 0b0001111100000000;
        for shift in 0..9 {
            let mask = straight_mask >> shift;
            if self.inner & mask == mask {
                return true;
            }
        }
        false
    }
    fn new(inner: u16) -> Self {
        Self {
            inner,
            _state: PhantomData,
        }
    }
}

pub fn highest_hand(cards: [Card; 7]) -> Hand {
    let color_bitmaps = convert_to_color_bitmaps(cards);

    let is_flush = color_bitmaps
        .iter()
        .any(|value_bitmap| value_bitmap.is_flush());

    todo!()
}

fn convert_to_color_bitmaps(cards: [Card; 7]) -> [ValueBitmap<SingleColored>; 4] {
    let mut color_bitmaps = [ValueBitmap::new(0); 4];

    for card in cards {
        color_bitmaps[card.color as usize].inner |= 1 << card.value as u8;
    }

    color_bitmaps
}
