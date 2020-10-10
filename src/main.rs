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
// raindrops_solver
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
/*
mod raindrops;
use crate::raindrops::raindrops_solver;
fn main() {

    let raindrop_number : u32 = 35;

    let drop_sound = raindrops_solver::raindrop_sound(raindrop_number);

    println!("{}",drop_sound);
}
*/
// Module 4:
// nth prime
//
// nth_prime_solver
// INPUT  || number : u8
// OUTPUT || number : u32
//
// Given a number n, determine what the nth prime is.
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime
// is 13 if your language provides methods in the standard library to deal with prime numbers, pretend they
// don't exist and implement them yourself.
/*
mod nth_prime;
use crate::nth_prime::nth_prime_solver;
fn main() {
s
    let nth : u8 = 10;

    let nth_prime_num = nth_prime_solver::nth_prime_calc(nth);

    println!("{} is the {}th prime number.", nth_prime_num, nth);

}
*/

// Module 5:
// bottles of beer song
//
// sing_beer_song
// prints the lyrics to the song "99 bottles of beer".
// INPUT  || beer_number : u8
// OUTPUT || NO RETURN, prints directly to console
//
// 99 bottles of beer on the wall, 99 bottles of beer.
// Take one down and pass it around, 98 bottles of beer on the wall.
//
// ...
//
// 2 bottles of beer on the wall, 2 bottles of beer.
// Take one down and pass it around, 1 bottle of beer on the wall.
//
// 1 bottle of beer on the wall, 1 bottle of beer.
// Take it down and pass it around, no more bottles of beer on the wall.
//
// No more bottles of beer on the wall, no more bottles of beer.
// Go to the store and buy some more, 99 bottles of beer on the wall.

/*
mod bottles_of_beer;
use crate::bottles_of_beer::bottles_of_beer_singer;
fn main() {
    let beer_number = 5;
    bottles_of_beer_singer::sing_beer_song(beer_number);
}

*/

// Module 6:
// proverb expander
//
// expand_proverb
// Given a list of inputs, generate the relevant proverb.
// INPUT  || nouns : []
// OUTPUT || NO RETURN, prints directly to console
//
// For want of a horseshoe nail, a kingdom was lost, or so the saying goes.
// Given a list of inputs, generate the relevant proverb. For example, given the list
// ["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"], you will output the full text
// of this proverbial rhyme:
//
// For want of a nail the shoe was lost.
// For want of a shoe the horse was lost.
// For want of a horse the rider was lost.
// For want of a rider the message was lost.
// For want of a message the battle was lost.
// For want of a battle the kingdom was lost.
// And all for the want of a nail.
//
// Note that the list of inputs may vary; your solution should be able to handle lists of arbitrary
// length and content. No line of the output text should be a static, unchanging string; all should
// vary according to the input given.
/*
mod proverb;
use crate::proverb::proverb_expander;

fn main() {

    let input_nouns = ["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"];

    proverb_expander::expand_proverb(&input_nouns);
}
*/

// Module 7:
// difference of squares
//
// solve_square_difference
// Find the difference between the square of the sum and the sum of the squares of the first N
// natural numbers.
//
// INPUT  || value : u32
//
// The square of the sum of the first ten natural numbers is (1 + 2 + ... + 10)² = 55² = 3025.
// The sum of the squares of the first ten natural numbers is 1² + 2² + ... + 10² = 385.
//
// Hence the difference between the square of the sum of the first ten natural numbers and the sum
// of the squares of the first ten natural numbers is 3025 - 385 = 2640.
/*
mod difference_of_squares;
use crate::difference_of_squares::square_difference_solver;

fn main () {
    let value : u32  = 10;
    let square_diff : u32 =  square_difference_solver::solve_square_difference(value);
    println!("For {}, the difference of squares is {}.", value, square_diff);
}
*/
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
/*
mod sum_of_multiples;
use crate::sum_of_multiples::sum_of_multiples_solver;

fn main() {
    let mut multiples : Vec<u32> = [3, 5].to_vec();
    let max_val : u32 = 20;
    let mult_sum : u32 = sum_of_multiples_solver::solve_sum_of_multiples(multiples.as_mut_slice(), max_val);
    println!("{}", mult_sum);
}
*/

// Graphical Testing

// extern crate termion;
//
// use termion::{color, style};
//
// use std::io;
//
// fn main() {
//     println!("{}Red", color::Fg(color::Red));
//     println!("{}Blue", color::Fg(color::Blue));
//     println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
//     println!("{}Just plain italic", style::Italic);
// }
extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    // import string array of module names
    // VERIFY THIS IS UP TO DATE!!!
    let mut exercisim_modules : [String; 8] = ["Hello World".to_string(),"Leap Year".to_string(),
                                                "Raindrops".to_string(),"Nth Prime".to_string(),
                                                "Beer Song".to_string(), "Proverb".to_string(),
                                                "Difference of Squares".to_string(),
                                                "Sum of Multiples".to_string()];

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
           "{}{}This is a one line test.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
            .unwrap();
    stdout.flush().unwrap();

    write!(stdout,
           "{}This is a two line test.{}",
           termion::cursor::Goto(1, 2),
           termion::cursor::Hide)
            .unwrap();
    stdout.flush().unwrap();

    write!(stdout,
           "{}This is a three line test.{}",
           termion::cursor::Goto(1, 3),
           termion::cursor::Hide)
            .unwrap();
    stdout.flush().unwrap();

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

/*
extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
           "{}{}q to exit. Type stuff, use alt, and so on.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
            .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1, 1),
               termion::clear::CurrentLine)
                .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
*/
