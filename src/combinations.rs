use std::array;

use crate::Card;

#[derive(Clone)]
pub struct CardCombinations<'a, const N: usize, const R: usize> {
    indices: [usize; R],
    cards: &'a [Card; N],
    first: bool,
}
impl<'a, const N: usize, const R: usize> CardCombinations<'a, N, R> {
    pub fn new(cards: &'a [Card; N]) -> Self {
        assert!(R <= N);

        Self {
            indices: array::from_fn(|index| index),
            cards,
            first: true,
        }
    }
    fn get_current_combination(&self) -> [Card; R] {
        array::from_fn(|index| self.cards[self.indices[index]])
    }
}
impl<const N: usize, const R: usize> Iterator for CardCombinations<'_, N, R> {
    type Item = [Card; R];
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.get_current_combination());
        }

        if increment_indices::<N, R>(&mut self.indices) {
            None
        } else {
            Some(self.get_current_combination())
        }
    }
}
        }

        // Return the current combination
        Some(self.get_current_combination())
    }
}
