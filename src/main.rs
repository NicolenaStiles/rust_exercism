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
// INPUT  || year : interger
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
}
*/
