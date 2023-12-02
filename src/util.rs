use std::path::Path;
pub fn get_input_filename(curr_filename: &str) -> &str {
    return Path::new(Path::new(curr_filename).file_name().unwrap()).file_stem().unwrap().to_str().unwrap();
}
