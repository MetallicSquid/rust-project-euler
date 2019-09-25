// Using names.txt (right click and 'Save Link/Target As...'), a 46K text file
// containing over five-thousand first names, begin by sorting it into
// alphabetical order. Then working out the alphabetical value for each name,
// multiply this value by its alphabetical position in the list to obtain a name
// score.

// For example, when the list is sorted into alphabetical order, COLIN, which is
// worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN
// would obtain a score of 938 × 53 = 49714.

// What is the total of all the name scores in the file?

use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn file_read() -> Result<(), Error> {
    let file = File::open("p022_names.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents_vec = Vec::new();
    for line in buf_reader.lines() {
        let content = String::from(line);
        let content_vec: Vec<&str> = content.split(",").collect();
        let 
    }
    Ok(())
}

fn main() {
    // let file = File::open("p022_names.txt");
    // let mut buf_reader = BufReader::new(file);
    // let mut contents = String::new();
    // buf_reader.read_to_string(&mut contents);
    // println!("File contents: {}", contents);
}
