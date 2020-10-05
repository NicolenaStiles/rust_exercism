// Module 8:
// Sum Of Multiples
//
// solve_sum_of_multiples
// Given a number, find the sum of all the unique multiples of particular numbers up
// to but not including that number.
//
// INPUT  || value : u32
//
// If we list all the natural numbers below 20 that are multiples of 3 or 5, we get
// 3, 5, 6, 9, 10, 12, 15, and 18.
// The sum of these multiples is 78.

use std::collections::HashMap;

pub fn solve_sum_of_multiples(mults : &[u32] , max : u32) {

    // variable initialization
    let mut list_of_multiples : Vec<u32> = Vec::new();
    let mut presence_of_multiples = HashMap::new();

    // for each item in the multiple stack
    for &n in mults {

        println!("DEBUG current mult is {}", n);
        let mut scalar : u32 = 1;
        let mut curr_value : u32 = n * scalar;

        // up to and not including the max value
        while (n * scalar < max) {
            println!("{} * {} is {} and is a multiple.", n, scalar, curr_value);
            if !presence_of_multiples.contains_key(&curr_value) {
                presence_of_multiples.insert(curr_value, true);
                list_of_multiples.push(curr_value);
            }
            scalar = scalar + 1;
            curr_value = n * scalar;
        }
    }

    // cleanup vector of multa
    list_of_multiples.sort();
}
