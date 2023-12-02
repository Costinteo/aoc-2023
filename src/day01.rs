use std::fs;

// taken from Stack Overflow
fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}

pub fn solve() {
    println!("Hello Day 1!");
    let contents = fs::read_to_string("res/input_01.txt").unwrap();
    let lines = words_by_line(&contents);

    println!("{:?}", lines);
}
