pub mod day01;
pub mod util;

const INPUT_FILEPATH: &str = "res/input_";

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let _ = day01::solve();

    Ok(())
}
