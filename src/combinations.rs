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

/// Calculates nCr
// Also, this is horribly inefficient, but its at compile time, so it doesnt matter.
// The upside is that this uses the same code as actually generating the combinations
pub const fn num_combinations<const N: usize, const R: usize>() -> usize {
    // Initialize indices
    let mut indices = [0; R];
    let mut i = 0;
    while i < R {
        indices[i] = i;
        i += 1;
    }

    let mut result = 1;
    while !increment_indices::<N, R>(&mut indices) {
        result += 1;
    }
    result
}

/// Increments the given indices, returns whether the end has been reached
const fn increment_indices<const N: usize, const R: usize>(indices: &mut [usize; R]) -> bool {
    let mut indices_index = R - 1;

    // Decrement indices index to find one that isnt at its maximum allowed value
    while indices[indices_index] == indices_index + N - R {
        if indices_index > 0 {
            indices_index -= 1;
        } else {
            // Last combination reached
            return true;
        }
    }

    // Increment the found index
    indices[indices_index] += 1;
    // And reset the ones to its right
    // Cannot use `for right_index in indices_index + 1..R` because it isnt const yet
    let mut right_index = indices_index + 1;
    while right_index < R {
        indices[right_index] = indices[right_index - 1] + 1;

        right_index += 1;
    }

    false
}
