// Let d(n) be defined as the sum of proper divisors of n (numbers less than n
// which divide evenly into n). If d(a) = b and d(b) = a, where a ≠ b, then a
// and b are an amicable pair and each of a and b are called amicable numbers.

// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44,
// 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4,
// 71 and 142; so d(284) = 220.

// Evaluate the sum of all the amicable numbers under 10000.

fn main() {
    let mut sum: i32 = 0;
    for a in 1..10001 {
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        for i in 1..a {
            if a % i == 0 {
                b += i;
            }
        }
        for j in 1..b {
            if b % j == 0 {
                c += j;
            }
        }
        if c == a && a != b {
            sum += a;
            sum += b;
        }
    }
    println!("The sum of amicable numbers below 10000: {}", sum / 2);
}
