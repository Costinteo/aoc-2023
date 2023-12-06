pub mod day01;
pub mod util;

const INPUT_FILEPATH: &str = "res/input_";

fn main() -> Result<(), std::io::Error> {
    let _ = day01::solve().expect("Error!");

    Ok(())
}
