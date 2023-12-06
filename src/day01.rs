use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

use crate::util;
use crate::INPUT_FILEPATH;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input_filename = format!("{}{}.txt", INPUT_FILEPATH, util::get_input_filename(file!()));
    let file = fs::File::open(&input_filename).expect(&format!("File {} not found!", input_filename));
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|res| res.expect("Unable to read lines!")).collect();
    let lines_ref = &lines;

    let mut sum: u32 = 0;

    // --== [ Fake sum for Task 1 ] ==--
    for line in lines_ref {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        let calibration_num: u32 = format!("{}{}", digits[0], digits[digits.len() - 1]).parse()?;
        sum += calibration_num;
    }

    println!("Fake sum: {}", sum);

    // --== [ Real sum for Task 2 ] ==--
    let str_to_digit_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        // hack to make the functional solution still work because I read the task wrong at first
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let mut sum: u32 = 0;

    for line in lines_ref {
        let map_ref = &str_to_digit_map;
        let unwrapped_line: &String = line;
        // Find all occurences of the tokens/keys in the line and map them to their values, return
        // tuple of format (index, decimal_value)
        let min_digits: Vec<(u32, &u32)> = map_ref.keys()
            .filter_map(|token| unwrapped_line.find(token).map(|index| (index as u32, map_ref.get(token).unwrap()))).collect();
        let max_digits: Vec<(u32, &u32)> = map_ref.keys()
            .filter_map(|token| unwrapped_line.rfind(token).map(|index| (index as u32, map_ref.get(token).unwrap()))).collect();
        let min_digit = min_digits.iter().min_by(|&a, &b| a.0.cmp(&b.0)).unwrap().1;
        let max_digit = max_digits.iter().max_by(|&a, &b| a.0.cmp(&b.0)).unwrap().1;
        println!("Min digit: {}, Max digit: {}, Line: {}", min_digit, max_digit, line);
        let calibration_num = min_digit * 10 + max_digit;
        sum += calibration_num;
    }
    
    println!("Real sum: {}", sum);

    Ok(())
}
