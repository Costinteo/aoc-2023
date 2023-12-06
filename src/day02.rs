use std::fs;
use std::io::{BufRead, BufReader};

use crate::util;
use crate::INPUT_FILEPATH;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input_filename = format!("{}{}.txt", INPUT_FILEPATH, util::get_input_filename(file!()));

    let file = fs::File::open(&input_filename).expect(&format!("File {} not found!", input_filename));
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|res| res.expect("Unable to read lines!")).collect();
    let lines_ref = &lines;

    // --==[ Task 1 ]==--
    let mut id_sum: u32 = 0;
    
    for line in lines_ref {
        let id: u32 = line.split_whitespace().nth(2).expect("Lines don't have at least two tokens")[..1].parse()?;
        println!("{}", id)
    }

    Ok(())
}
