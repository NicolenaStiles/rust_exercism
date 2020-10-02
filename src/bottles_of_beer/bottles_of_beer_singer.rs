// Module 5:
// bottles of beer song
//
// sing_beer_song
// prints the lyrics to the song "99 bottles of beer".
// INPUT  || beer_number : u8
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

        // basic decreasing for loop for lines n -> 3
        let mut i = input;
        let end = 2;
        let step = 1;
        while i > end {
            println!("{} bottles of beer on the wall, {} bottles of beer.", i, i);
            println!("Take one down and pass it around, {} bottles of beer on the wall.", i);
            i -= step;
        }
    }

    // the rest of the unique lines
    if input >= 2 {
        println!("2 bottles of beer on the wall, 2 bottles of beer.");
        println!("Take one down and pass it around, 1 bottle of beer on the wall.");
    }

    if input >= 1 {
        println!("1 bottle of beer on the wall, 1 bottle of beer.");
        println!("Take it down and pass it around, no more bottles of beer on the wall.");
    }

    println!("No more bottles of beer on the wall, no more bottles of beer.");
    println!("Go to the store and buy some more, 99 bottles of beer on the wall.");

}
