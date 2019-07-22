// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

// a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 52.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn main() {
    'outer: for a in 1..1001 {
        for b in 1..1001 {
            for c in 1..1001 {
                let lhs: i64 = (a * a) + (b * b);
                let rhs: i64 = c * c;
                let sum: i64 = a + b + c;
                let product: i64 = a * b * c;

                if lhs == rhs && sum == 1000 {
                    println!("{}", product);
                    break 'outer;
                }
            }
        }
    }
}
