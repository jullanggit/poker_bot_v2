use std::{array, ops::Index};

use crate::Hand;

/// An iterator that returns all possible length R combinations of 0..N
#[derive(Clone)]
pub struct Combinations<const N: usize, const R: usize> {
    indices: [usize; R],
    first: bool,
}
impl<const N: usize, const R: usize> Default for Combinations<N, R> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const N: usize, const R: usize> Combinations<N, R> {
    pub fn new() -> Self {
        assert!(R <= N);

        Self {
            indices: array::from_fn(|index| index),
            first: true,
        }
    }
}
impl<const N: usize, const R: usize> Iterator for Combinations<N, R> {
    type Item = [usize; R];
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.indices);
        }

        if increment_indices::<N, R>(&mut self.indices) {
            None
        } else {
            Some(self.indices)
        }
    }
}

/// Calculates nCr
pub const fn num_combinations(n: usize, r: usize) -> usize {
    if r > n {
        0
    } else {
        // Const version of this: (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
        let mut result = 1;

        let mut i = 0;
        while i < r {
            result *= n - i;
            result /= i + 1;

            i += 1;
        }
        result
    }
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

pub struct CombinationMap<const N: usize, const R: usize>
where
    [(); num_combinations(N, R)]:,
    [[(); R - 1]; N - 1]:,
{
    /// An array containing the player hand, in lexicographic order by the combination that produced it
    pub array: [Hand; num_combinations(N, R)],
    precomputed_num_combinations: [[usize; R - 1]; N - 1],
}
impl<const N: usize, const R: usize> Default for CombinationMap<N, R>
where
    [(); num_combinations(N, R)]:,
    [[(); R - 1]; N - 1]:,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, const R: usize> CombinationMap<N, R>
where
    [(); num_combinations(N, R)]:,
    [[(); R - 1]; N - 1]:,
{
    pub const fn new() -> Self {
        Self {
            array: [Hand::RoyalFlush; num_combinations(N, R)],
            precomputed_num_combinations: precompute_num_combinations::<N, R>(),
        }
    }
}
impl<const N: usize, const R: usize> Index<[usize; R]> for CombinationMap<N, R>
where
    [(); num_combinations(N, R)]:,
    [[(); R - 1]; N - 1]:,
{
    type Output = Hand;
    fn index(&self, combination_index: [usize; R]) -> &Self::Output {
        let mut index = 0;

        for (position, value) in combination_index.into_iter().enumerate() {
            let min_value = if position > 0 {
                combination_index[position - 1] + 1
            } else {
                0
            };

            for smaller_value in min_value..value {
                // TODO: Maybe invert this nested array, so that the index changing
                // more frequently (smaller_value) is the inner one (better for cache)
                index += self.precomputed_num_combinations[N - 1 - smaller_value][R - 1 - position];
            }
        }

        &self.array[index]
    }
}

const fn precompute_num_combinations<const N: usize, const R: usize>() -> [[usize; R - 1]; N - 1] {
    let mut result = [[0; R - 1]; N - 1];

    let mut n = 0;
    while n < N - 1 {
        let mut r = 0;
        while r < R - 1 {
            result[n][r] = num_combinations(n, r);

            r += 1;
        }

        n += 1
    }

    result
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
                        assert_eq!(num_combinations($num, $num), 1);
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
                        assert_eq!(num_combinations($N,$R), $E);
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
