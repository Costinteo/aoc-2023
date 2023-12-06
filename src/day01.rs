use std::fs;
use std::io::{self, BufRead, BufReader};

use crate::util;
use crate::INPUT_FILEPATH;

pub fn solve() -> Result<(), io::Error> {
    let input_filename = format!("{}{}.txt", INPUT_FILEPATH, util::get_input_filename(file!()));

    println!("Reading [{}]...", input_filename);
    let file = fs::File::open(input_filename).expect("File not found!");
    let reader = BufReader::new(file);
    //println!("Reading [{}]...", input_filename);

    let mut cnt = 0;
    for _line in reader.lines() {
        cnt += 1;
    }
    
    println!("Found {:?} lines!", cnt);

    Ok(())
}
