use std::fs;
use std::io::{BufRead, BufReader};

use crate::util;
use crate::INPUT_FILEPATH;
use crate::util::Mat;

const NORMAL_CHARSET: &str = ".0123456789";
fn is_anomalous(c: char) -> bool { NORMAL_CHARSET.chars().find(|x| *x == c).is_none() }
fn is_num(c: char) -> bool { NORMAL_CHARSET.chars().skip_while(|x| *x == '.').find(|x| *x == c).is_some() }

fn is_anomaly_around(m: &Mat<char>, row_start: usize, col_start: usize) -> (bool, usize) {
    let mut word_len: usize = 0;

    // Find word length
    while col_start + word_len < m.get_width() && is_num(*(m.get(row_start, col_start + word_len).unwrap())) {
        word_len += 1;
    }

    // Calculate all coords around the word as signed integer
    // We can then check each coord in the Matrix
    // But only after passing checks to be in bounds (non-negative product)
    let col_before: i32 = col_start as i32 - 1;
    let col_after: i32 = col_before + word_len as i32 + 1;
    let row_before: i32 = row_start as i32 - 1;
    let row_after: i32 = (row_start + 1) as i32;

    for row in row_before..(row_after + 1) {
        for col in col_before..(col_after + 1) {
            if row * col >= 0 && (row as usize) < m.get_height() && (col as usize) < m.get_width() {
                let row = row as usize;
                let col = col as usize;
                // Check if char found at coords (row, col) is anomalous
                if is_anomalous(*(m.get(row, col).unwrap())) { return (true, word_len); }
            }
        }
    }

    return (false, word_len);
}

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input_filename = format!("{}{}.txt", INPUT_FILEPATH, util::get_input_filename(file!()));

    let file = fs::File::open(&input_filename).expect(&format!("File {} not found!", input_filename));
    let reader = BufReader::new(file);
	let lines: Vec<String> = reader.lines().map(|res| res.expect("Unable to read lines!")).collect();
    let lines_ref = &lines;

    let cols = lines_ref[0].chars().count();
    let rows = lines_ref.iter().count();


    let mut matrix: Mat<char> = Mat::new(rows, cols, '.');

    for (row, line) in lines_ref.iter().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            matrix.set(row, col, chr).expect("Out of bounds write in Matrix!");
        }
    }

    //println!("{}", matrix);

    let mut part_sum: u32 = 0;
    let mut row: usize = 0;
    while row < matrix.get_height() {
        let mut col: usize = 0;
        while col < matrix.get_width() {
            // Find numbers
            if is_num(*(matrix.get(row, col).unwrap())) {
                let (anomaly_found, word_len) = is_anomaly_around(&matrix, row, col);
                if anomaly_found {
                    let s: String = matrix.get_slice(row, col, col + word_len - 1).unwrap().into_iter().collect::<String>();
                    // println!("String: {:?}, len: {}", s, word_len);
                    let num: u32 = s.parse().unwrap();
                    part_sum += num;

                    // Skip the number found when parsing
                }
                col += word_len;
                continue;
            }
            col += 1;
        }
        row += 1;
    }
    
    println!("Part sum is: {}", part_sum);
    Ok(())
}
