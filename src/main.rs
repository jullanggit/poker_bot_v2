use poker_bot_v2::{calculate, calculate_7, io::get_cards};

/// Runs calculate with the given const lens
macro_rules! match_len_calculate {
    ($present_cards:expr, $($num:expr),+) => {
        match $present_cards.len() {
            $(
                $num => {
                    let present_cards = $present_cards.try_into().unwrap();
                    calculate::<$num>(present_cards)
                }
            ),+
            _ => unreachable!()
        }
    };
}
fn main() {
    let cards = get_cards().unwrap();

    let results = if cards.len() != 7 {
        match_len_calculate!(cards, 2, 3, 4, 5, 6)
    } else {
        calculate_7(cards.try_into().unwrap())
    };
    println!("Results: {results:?}");
}
