// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

// What is the 10 001st prime number?

fn main() {
    let mut flag: bool = true;
    let mut current_number: i64 = 2;
    let mut count: i64 = 2;
    let mut sec_count: i32 = 0;
    
    while flag == true {
        if count != current_number && current_number % count == 0 {
            current_number += 1;
            count = 2;
        } else if count != current_number {
            count += 1;
        } else {
            sec_count += 1;
            if sec_count == 10001 {
                println!("{}", current_number);
                flag = false;
            }
            current_number += 1;
            count = 2;
        }
    }
}
