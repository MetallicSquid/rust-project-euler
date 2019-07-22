// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main() {
    let mut flag: bool = true;
    let mut number_count: i64 = 1;
    let mut divisor_count: i8 = 1;

    while flag == true {
        if number_count % divisor_count as i64 == 0 {
            if divisor_count == 20 {
                println!("{}", number_count);
                flag = false;
            }
            divisor_count += 1;
        } else {
            number_count += 1;
            divisor_count = 1;
        }
    }
}
