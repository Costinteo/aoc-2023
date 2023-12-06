#!/usr/bin/env bash

echo "Generating day$1.rs..."

cat << _END_OF_FILE > src/day$1.rs
use std::fs;
use std::io::{self, BufRead, BufReader};

use crate::util;
use crate::INPUT_FILEPATH;

pub fn solve() -> Result<(), io::Error> {
    let input_filename = format!("{}{}.txt", INPUT_FILEPATH, util::get_input_filename(file!()));

    let file = fs::File::open(&input_filename).expect(&format!("File {} not found!", input_filename));
    let reader = BufReader::new(file);

    Ok(())
}
_END_OF_FILE
