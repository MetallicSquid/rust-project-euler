fn main() {
    let mut flag: bool = true;
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut even_total: i32 = 0;

    while flag == true {
        if a > b {
            if a >= 4000000 {
                flag = false;
            }
            b += a;
            if a % 2 == 0 {
                even_total += a;
            }
        }
        if b > a {
            if b >= 4000000 {
                flag = false;
            }
            a += b;
            if b % 2 == 0 {
                even_total += b;
            }
        }
    }
    println!("Total of even numbers: {}", even_total); 
}
