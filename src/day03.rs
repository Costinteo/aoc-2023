use std::fs;
use std::io::{BufRead, BufReader};

use crate::util;
use crate::INPUT_FILEPATH;
use crate::util::Mat;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input_filename = format!("{}{}.txt", INPUT_FILEPATH, util::get_input_filename(file!()));

    let file = fs::File::open(&input_filename).expect(&format!("File {} not found!", input_filename));
    let reader = BufReader::new(file);
	let lines: Vec<String> = reader.lines().map(|res| res.expect("Unable to read lines!")).collect();
    let lines_ref = &lines;

    let cols = lines_ref[0].chars().count();
    let rows = lines_ref.iter().count();


    let mut matrix: Mat<char> = Mat::new(rows, cols, '.');

    //println!("{}", matrix);

    Ok(())
}
