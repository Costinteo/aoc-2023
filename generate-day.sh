#!/usr/bin/env bash

echo "Generating day$1.rs..."

cat << _END_OF_FILE > src/day$1.rs
use std::fs;
use std::io::{BufRead, BufReader};

use crate::util;
use crate::INPUT_FILEPATH;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input_filename = format!("{}{}.txt", INPUT_FILEPATH, util::get_input_filename(file!()));

    let file = fs::File::open(&input_filename).expect(&format!("File {} not found!", input_filename));
    let reader = BufReader::new(file);
	let lines: Vec<String> = reader.lines().map(|res| res.expect("Unable to read lines!")).collect();

    Ok(())
}
_END_OF_FILE
