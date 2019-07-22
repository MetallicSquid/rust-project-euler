// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is 9009 = 91 × 99.

// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    let mut current: i32 = 0;

    for i in 100..1000 {
        for j in 100..1000 {
            let prod: i32 = i * j;
            let prod_string = prod.to_string();
            let mut rev_string = String::from("");
            let mut prod_vector = Vec::new();

            for c in prod_string.chars() {
                prod_vector.push(c);
            }
            for n in (0..prod_vector.len()).rev() {
                rev_string.push(prod_vector[n]);
            }
            if rev_string == prod_string {
                if prod_string.parse::<i32>().unwrap() > current {
                    current = prod_string.parse::<i32>().unwrap();
                }
            }
        }
    }
    println!("{}", current);
}