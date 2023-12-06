pub mod day01;
pub mod day02;
pub mod util;

use std::env;

const INPUT_FILEPATH: &str = "res/input_";
const USAGE: &str = "Usage: cargo run DAY_NUM";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let day_num: u32 = args.get(1).expect(USAGE).parse().expect(USAGE);
    
    // This can also be done with a HashMap, unsure which is nicer
    match day_num {
        1 => day01::solve().expect("Error!"),
        2 => day02::solve().expect("Error!"),
        _ => println!("Day {} not implemented yet!", day_num),
    }

    Ok(())
}
