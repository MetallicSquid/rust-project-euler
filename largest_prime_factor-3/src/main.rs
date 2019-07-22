fn main() {
    let mut target: i64 = 600851475143;
    let mut flag: bool = true;
    let mut current_number: i64 = 2;
    let mut current_prime: i64 = 0;
    let mut largest_prime: i64 = 0;
    let mut count: i64 = 2;

    while flag == true {
        if count != current_number && current_number % count == 0 {
            current_number += 1;
            count = 2;
        } else if count != current_number {
            count += 1;
        } else {
            current_prime = current_number;  
            if target % current_prime == 0 {
                target = target / current_prime;
                if largest_prime < current_prime {
                    largest_prime = current_prime;
                }
            }
            current_number += 1;
            count = 2;
            if current_prime >= target {
                flag = false;
            }
        }
    } 
    println!("Largest prime factor: {}", largest_prime);
}