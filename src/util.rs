use std::path::Path;
pub fn get_input_filename(curr_filename: &str) -> &str {
    let filename = Path::new(curr_filename).file_stem().unwrap().to_str().unwrap();
    let day_num = &filename[filename.len() - 2..];
    return day_num
}
