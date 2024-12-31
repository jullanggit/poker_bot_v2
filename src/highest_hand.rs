use crate::{Card, Hand};

#[derive(Clone, Copy)]
struct ValueBitmap(u16);
impl ValueBitmap {
    fn is_flush(&self) -> bool {
        self.0.count_ones() >= 5
    }
}

pub fn highest_hand(cards: [Card; 7]) -> Hand {
    let color_bitmaps = convert_to_color_bitmaps(cards);

    let is_flush = color_bitmaps
        .iter()
        .any(|value_bitmap| value_bitmap.is_flush());

    todo!()
}

fn convert_to_color_bitmaps(cards: [Card; 7]) -> [ValueBitmap; 4] {
    let mut color_bitmaps = [ValueBitmap(0); 4];

    for card in cards {
        color_bitmaps[card.color as usize].0 |= 1 << card.value as u8;
    }

    color_bitmaps
}
