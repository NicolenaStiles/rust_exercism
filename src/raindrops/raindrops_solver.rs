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

pub fn raindrop_sound(input : u32) -> String {

    // initalize varible to hold status of leap evaluation
    let mut sounds = String::new();

    // flag to see if any factors are met
    let mut tripped : bool = false;

    // applying sound parameters
    if input % 3 == 0 {
        tripped = true;
        sounds = sounds + "Pling";
    }

    if input % 5 == 0 {
        tripped = true;
        sounds = sounds + "Plang";
    }

    if input % 7 == 0 {
        tripped = true;
        sounds = sounds + "Plong";
    }

    // if no conditions were met, return the number
    if tripped == false {
        sounds = input.to_string();
    }

    // pass result back to calling function
    sounds
}
