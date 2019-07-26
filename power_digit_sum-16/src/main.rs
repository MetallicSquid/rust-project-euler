// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

// What is the sum of the digits of the number 2^1000?

fn power(num: i128, power: i128) -> i128 {
    let mut current: i128 = num;
    for n in 1..power {
        current *= num;
    }
    current
}

fn main() {
    println!("{}", power(2, 25));
}
