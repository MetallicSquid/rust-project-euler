// If the numbers 1 to 5 are written out in words: one, two, three, four, five,
// then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

// If all the numbers from 1 to 1000 (one thousand) inclusive were written out
// in words, how many letters would be used?

// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and
// forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20
// letters. The use of "and" when writing out numbers is in compliance with
// British usage.
use std::collections::HashMap;

fn main() {
    let mut total: i32 = 0;
    
    let mut prim_letters = HashMap::new();
    // All correct.
    prim_letters.insert(1, 3);
    prim_letters.insert(2, 3);
    prim_letters.insert(3, 5);
    prim_letters.insert(4, 4);
    prim_letters.insert(5, 4);
    prim_letters.insert(6, 3);
    prim_letters.insert(7, 5);
    prim_letters.insert(8, 5);
    prim_letters.insert(9, 4);
    
    let mut teen_letters = HashMap::new();
    // All correct.
    teen_letters.insert(1, 6);
    teen_letters.insert(2, 6);
    teen_letters.insert(3, 8);
    teen_letters.insert(4, 8);
    teen_letters.insert(5, 7);
    teen_letters.insert(6, 7);
    teen_letters.insert(7, 9);
    teen_letters.insert(8, 8);
    teen_letters.insert(9, 8);

    let mut tens_letters = HashMap::new();
    // All correct.
    tens_letters.insert(1, 3);
    tens_letters.insert(2, 6);
    tens_letters.insert(3, 6);
    tens_letters.insert(4, 5);
    tens_letters.insert(5, 5);
    tens_letters.insert(6, 5);
    tens_letters.insert(7, 7);
    tens_letters.insert(8, 6);
    tens_letters.insert(9, 6);

    let mut hund_letters = HashMap::new();
    // All correct.
    hund_letters.insert(1, 10);
    hund_letters.insert(2, 10);
    hund_letters.insert(3, 12);
    hund_letters.insert(4, 11);
    hund_letters.insert(5, 11);
    hund_letters.insert(6, 10);
    hund_letters.insert(7, 12);
    hund_letters.insert(8, 12);
    hund_letters.insert(9, 11);

    for n in 1..2 {
        let target = 115.to_string();
        println!("Target: {}", target);
        let mut target_vec: Vec<&str> = target.split("").collect();
        target_vec.remove(0);
        target_vec.remove(target_vec.len() - 1);
        'outer: for n in 0..target_vec.len() {
            if target_vec.len() == 1 {
                println!("Number: {}, Letters: {}", target_vec[n], prim_letters.entry(target_vec[n].parse::<i32>().unwrap()).or_insert(0));
                total += *prim_letters.entry(target_vec[n].parse::<i32>().unwrap()).or_insert(0);
            } else if target_vec.len() == 2 {
                if n == 0 && target_vec[n] == "1" && target_vec[n + 1] != "0" {
                    println!("Teen Number: {}, Letters: {}", target_vec[n + 1], teen_letters.entry(target_vec[n + 1].parse::<i32>().unwrap()).or_insert(0));
                    total += *teen_letters.entry(target_vec[n + 1].parse::<i32>().unwrap()).or_insert(0);
                    break 'outer;
                } else if n == 0 {
                    println!("Tens Number: {}, Letters: {}", target_vec[n], tens_letters.entry(target_vec[n].parse::<i32>().unwrap()).or_insert(0));
                    total += *tens_letters.entry(target_vec[n].parse::<i32>().unwrap()).or_insert(0);
                } else if n == 1 {
                    println!("Number: {}, Letters: {}", target_vec[n], prim_letters.entry(target_vec[n].parse::<i32>().unwrap()).or_insert(0));
                    total += *prim_letters.entry(target_vec[n].parse::<i32>().unwrap()).or_insert(0);
                }
            } else if target_vec.len() == 3 {
                if n == 0 {
                    println!("Hund Number: {}, Letters: {}", target_vec[n], hund_letters.entry(target_vec[n].parse::<i32>().unwrap()).or_insert(0));
                    total += *hund_letters.entry(target_vec[n].parse::<i32>().unwrap()).or_insert(0);
                } else if n == 1 {
                    if target_vec[n] == "1" && target_vec[n + 1] != "0" {
                        println!("Teen Number: {}, Letters: {}", target_vec[n + 1], teen_letters.entry(target_vec[n + 1].parse::<i32>().unwrap()).or_insert(0));
                        total += *teen_letters.entry(target_vec[n + 1].parse::<i32>().unwrap()).or_insert(0);
                        if target_vec[1] != "0" && target_vec[2] != "0" {
                            println!("And (+3)");
                            total += 3;
                        }
                        break 'outer;
                    } else {
                        println!("Tens Number: {}, Letters: {}", target_vec[n], tens_letters.entry(target_vec[n].parse::<i32>().unwrap()).or_insert(0));
                        total += *tens_letters.entry(target_vec[n].parse::<i32>().unwrap()).or_insert(0);
                    }
                } else if n == 2 {
                    println!("Number: {}, Letters: {}", target_vec[n], prim_letters.entry(target_vec[n].parse::<i32>().unwrap()).or_insert(0));
                    total += *prim_letters.entry(target_vec[n].parse::<i32>().unwrap()).or_insert(0);
                    if target_vec[1] != "0" && target_vec[2] != "0" {
                        println!("And (+3)");
                        total += 3;
                    }
                }
            } else if target_vec.len() == 4 && n == 3 {
                total += 11;
            }
        }
        println!("Sub Total: {}", total);
    }
    println!("Total: {}", total);
}
