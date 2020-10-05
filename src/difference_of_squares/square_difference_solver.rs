// Module 7:
// difference of squares
//
// solve_square_difference
// Find the difference between the square of the sum and the sum of the squares of the first N
// natural numbers.
//
// INPUT  || NO INPUT
// OUTPUT || NO RETURN, prints directly to console
//
// The square of the sum of the first ten natural numbers is (1 + 2 + ... + 10)² = 55² = 3025.
// The sum of the squares of the first ten natural numbers is 1² + 2² + ... + 10² = 385.
//
// Hence the difference between the square of the sum of the first ten natural numbers and the sum
// of the squares of the first ten natural numbers is 3025 - 385 = 2640.
pub fn solve_square_difference(top_val : u32) -> u32 {

    let mut sum_of_square : u32 = 0;
    let mut square_of_sum : u32 = 0;

    // pad the input value by one because of how for loops in rust work
    for n in 0..top_val+1 {
        sum_of_square = sum_of_square + n.pow(2);
        square_of_sum = square_of_sum + n;
    }

    square_of_sum = square_of_sum.pow(2);

    let diff_of_squares : u32 = square_of_sum - sum_of_square;

    diff_of_squares
}
