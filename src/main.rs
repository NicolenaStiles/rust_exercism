// Rust on Exercism
//
// Remove block comment around module code to run
//
// Author: Nicolena Stiles
// Started September 2020

// Module 1:
// A basic "Hello, world!" program
/*
mod hello_world;
use crate::hello_world::hello_world_printer;
fn main() {
    hello_world_printer::sub();
}
*/

// Module 2:
// A leap year calculation program
//
// is_leap
// INPUT  || year : u32
// OUTPUT || leap : boolean
//
// Leap years in the first half of the 21st century is therefore
// 2000, 2004, 2008, 2012, 2016, 2020, 2024, 2028, 2032, 2036, 2040, 2044, and 2048
/*
mod leap_year;
use crate::leap_year::leap_year_solver;
fn main() {

    let test_year : u32 = 2048;

    let check_year = leap_year_solver::is_leap(test_year);

    println!("{}",check_year);
}*/

// Module 3:
// Raindrops (a variation on fizzbuzz)
//
// raindrop_sound
// INPUT  || number : u32
// OUTPUT || sounds : string
//
// Rules for Raindrops:
// 1. if it has 3 as a factor, add 'Pling'
// 2. if it has 5 as a factor, add 'Plang'
// 3. if it has 7 as a factor, add 'Plong'
// 4. NOT 3, 5, or 7 as factors, result is just the number itself
//
// Examples: 35 has 5 and 7 as factors
//           2048 has none of the factors

mod raindrops;
use crate::raindrops::raindrops_solver;
fn main() {

    let raindrop_number : u32 = 35;

    let drop_sound = raindrops_solver::raindrop_sound(raindrop_number);

    println!("{}",drop_sound);
}
