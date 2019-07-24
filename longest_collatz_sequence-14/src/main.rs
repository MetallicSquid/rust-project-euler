// The following iterative sequence is defined for the set of positive integers:

// n → n/2 (n is even)
// n → 3n + 1 (n is odd)

// Using the rule above and starting with 13, we generate the following sequence:

// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

// Which starting number, under one million, produces the longest chain?

// NOTE: Once the chain starts the terms are allowed to go above one million.

fn main() {
    let mut longest: i64 = 0;
    let mut longest_starter: i64 = 0;
    for n in 2..1000000 {
        let mut flag: bool = true;
        let starter: i64 = n;
        let mut current: i64 = n;
        let mut length: i64 = 0;

        while flag == true {
            if current % 2 == 0 {
                current = current / 2;
                length += 1;
            } else {
                if current == 1 {
                    if length > longest {
                        longest = length;
                        longest_starter = starter;
                    }
                    flag = false;
                }
                current = (3 * current) + 1;
                length += 1;
            }
        }
    }
    println!("Longest: {} at a chain length of {}.", longest_starter, longest);
}
