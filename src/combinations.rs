use std::{array, ops::Index};


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

struct CombinationMap<const N: usize, const R: usize, T>
where
    [(); num_combinations::<N, R>()]:,
{
    array: [T; num_combinations::<N, R>()],
}
impl<const N: usize, const R: usize, T> Index<[usize; R]> for CombinationMap<N, R, T>
where
    [(); num_combinations::<N, R>()]:,
{
    type Output = T;
    fn index(&self, index: [usize; R]) -> &Self::Output {
        let mut rank = 0;

        for (position, value) in index.into_iter().enumerate() {
            // For every legal value smaller than the current one
            for smaller_value in (if position > 0 {
                index[position - 1] + 1
            } else {
                0
            })..value
            {
                rank += num_combinations::<{ N - smaller_value - 1 }, { R - position - 1 }>()
            }
        }
        &self.array[rank]
    }
}

#[cfg(test)]
mod tests {
    mod increment_indices {
        use std::array;

        use crate::combinations::increment_indices;

        #[test]
        fn basic() {
            macro_rules! increment_indices {
                ($N:literal,$R:literal,$expected:expr) => {
                    // I know this is just manual snapshot testing, but insta kinda doesnt play well with loops & macros
                    let mut indices: [usize; $R] = array::from_fn(|index| index);

                    let mut i = 0;
                    // Just a while loop with the condition at the end
                    loop {
                        assert_eq!(indices, $expected[i]);
                        i += 1;

                        if !increment_indices::<$N, $R>(&mut indices) { break; }
                    }
                };
            }

            // I know this is just manual snapshot testing, but insta kinda doesnt play well with loops & macros
            increment_indices!(4, 2, [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]]);
            increment_indices!(5, 3, [
                [0, 1, 2],
                [0, 1, 3],
                [0, 1, 4],
                [0, 2, 3],
                [0, 2, 4],
                [0, 3, 4],
                [1, 2, 3],
                [1, 2, 4],
                [1, 3, 4],
                [2, 3, 4],
            ]);
        }
    }
    mod num_combinations {
        use crate::combinations::num_combinations;

        #[test]
        fn equal_n_r() {
            macro_rules! equal_n_r {
                ($($num:literal),+) => {
                    $(
                        assert_eq!(num_combinations::<$num, $num>(), 1);
                    )+
                };
            }
            equal_n_r!(1, 2, 3, 5, 8, 13, 21, 34, 55);
        }
        #[test]
        fn different_n_r() {
            macro_rules! different_n_r {
                ($(($N:literal, $R:literal, $E:literal)),+) => {
                    $(
                        assert_eq!(num_combinations::<$N, $R>(), $E);
                    )*
                };
            }
            different_n_r!(
                // Fibonacci :)
                (2, 1, 2),
                (5, 3, 10),
                (13, 8, 1287),
                (6, 4, 15),
                (47, 4, 178365)
            );
        }
    }
}
