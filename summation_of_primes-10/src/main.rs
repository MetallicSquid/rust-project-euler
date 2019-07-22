// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

// Find the sum of all the primes below two million.

// THIS IS SHOCKINGLY SLOW AND NEEDS TO BE REFACTORED.

fn main() {
    let mut flag: bool = true;
    let mut current_number: i64 = 2;
    let mut count: i64 = 2;
    let mut total: i64 = 0;
    
    while flag == true {
        if count != current_number && current_number % count == 0 {
            current_number += 1;
            count = 2;
        } else if count != current_number {
            count += 1;
        } else {
            // println!("{}", current_number);
            if current_number < 2000000 {
                total += current_number;
            } else {
                flag = false;
            }
            current_number += 1;
            count = 2;
        }
    }
    println!("{}", total);
}
