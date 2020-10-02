// Module 5:
// bottles of beer song
//
// nth_prime_solver
// INPUT  || number : u8
// OUTPUT || number : u32
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

pub fn sing_beer_song(input : u8) {
    if input >= 3 {
        println!("hi");
        for n in input..3 {
            println!("{}", n);
        }
    }

    /*
    println!("2 bottles of beer on the wall, 2 bottles of beer.");
    println!("Take it down and pass it around, no more bottles of beer on the wall.");

    println!("1 bottle of beer on the wall, 1 bottle of beer.");
    println!("Take it down and pass it around, no more bottles of beer on the wall.");

    println!("No more bottles of beer on the wall, no more bottles of beer.");
    println!("Go to the store and buy some more, 99 bottles of beer on the wall.");
    */

}
