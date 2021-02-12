// Module 9
// Grains calculator
//
// Calculate the number of grains of wheat on a chessboard given that the number on each square
// doubles.
//
// There once was a wise servant who saved the life of a prince. The king promised to pay whatever the
// servant could dream up. Knowing that the king loved chess, the servant told the king he would like
// to have grains of wheat. One grain on the first square of a chess board, with the number of grains
// doubling on each successive square.
//
// There are 64 squares on a chessboard (where square 1 has one grain, square 2 has two grains, and so
// on).
//
// Write code that shows:
//
// 1. how many grains were on a given square, and
// 2. the total number of grains on the chessboard


// TODO: FIX! This doesn't work, they're backwards for some reason.
pub fn calculate_square_grains (square_num : u32) -> u128 {
    let mut grains_on_square : u128 = 1;
    if square_num < 1 || square_num > 64 {
        println!("Invalid entry! Please enter a number between 1 and 64.");
    }
    else {
        grains_on_square = grains_on_square << (square_num - 1);
    }

    return grains_on_square;
}

pub fn calculate_total_grains (square_num : u32) -> u128 {
    let mut curr_grains : u128 = 1;
    let mut total_grains : u128 = 0;
    if square_num < 1 || square_num > 64 {
        println!("Invalid entry! Please enter a number between 1 and 64.");
    }
    else {
        for x in 1..=square_num {
            total_grains = total_grains + (curr_grains << (x - 1));
        }
    }

    return total_grains;
}
