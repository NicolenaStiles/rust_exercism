// Module 4:
// nth prime
//
// nth_prime_solver
// INPUT  || number : u8
// OUTPUT || number : u32
//
// Given a number n, determine what the nth prime is.
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime
// is 13 if your language provides methods in the standard library to deal with prime numbers,
// pretend they don't exist and implement them yourself.

fn is_prime(x : u32) -> bool {

    // primes are only divisible by 1 and itself, so try all
    // n-1 options (I know this isn't efficient)
    let mut divider : u32 = 2;

    while divider < x {
        if x % divider == 0{
            // using direct returns here because this is one of the cases where
            // it makes sense to exit early
            return false;
        } else {
            divider = divider + 1;
        }
    }

    return true;
}

pub fn nth_prime_calc(input : u8) -> u32 {

    // variable initalization
    let mut prime_count : u8 = 1;
    let mut curr_num : u32 = 3;

    //while (prime_count < input)
    while prime_count < input {

        let prime_status : bool = is_prime(curr_num);

        if prime_status == true {
                prime_count = prime_count + 1;
        }

        curr_num = curr_num + 1;
    }

    curr_num = curr_num - 1;
    curr_num
}
