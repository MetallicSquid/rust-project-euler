// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

// What is the sum of the digits of the number 2^1000?

// fn power(num: i128, power: i128) -> i128 {
//     let mut current: i128 = num;
//     for n in 1..power {
//         current *= num;
//     }
//     current
// }

fn main() {
    let mut num_vec = Vec::new();
    let mut flag: bool = true;
    num_vec.push("2");
    while flag == true {
        for n in (0..num_vec.len()).rev() {
            let current: i8 = num_vec[n].parse::<i8>().unwrap();
            current *= 2;
            // num_vec = current.to_string().split("").collect();
            if current > 9 {
                let carry: i8 = current % 10;
                let number: i8 = num_vec[n] + carry;
                num_vec.remove(n);
                num_vec.insert(n, number); 
            }

        }
        num_vec.clear();
    }
}
