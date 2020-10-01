// Leap Year solver
// Takes input from the command line and determines if the designated year
// is a leap year

pub fn is_leap(year : u32) -> bool {

    // initalize varible to hold status of leap evaluation
    let mut leap : bool = false;

    // applying leap year calc parameters
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                leap = true;
            }
        } else {
            leap = true;
        }
    }
    // returning the value
    leap
}
