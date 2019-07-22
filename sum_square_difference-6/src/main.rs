// The sum of the squares of the first ten natural numbers is,
// 12 + 22 + ... + 102 = 385

// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)2 = 552 = 3025

// Hence the difference between the sum of the squares of the first ten natural numbers 
// and the square of the sum is 3025 − 385 = 2640.

// Find the difference between the sum of the squares of the first one hundred natural 
// numbers and the square of the sum.

fn main() {
    let mut sum_of_squares: i32 = 0;
    let mut square_of_sums: i32 = 0;

    for n in 1..101 {
        sum_of_squares += n * n;
    }
    for n in 1..101 {
        square_of_sums += n;
    }
    square_of_sums *= square_of_sums;

    println!("{}", square_of_sums - sum_of_squares);
}
