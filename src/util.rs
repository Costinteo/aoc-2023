use std::path::Path;
use std::fmt;

#[derive(Debug)]
pub enum MatError {
    OutOfBounds,
}


pub struct Mat<T> {
    internal_vec: Vec<T>,
    rows: usize,
    cols: usize
}

impl<T: fmt::Display + Copy> Mat<T> {
    pub fn new(rows: usize, cols: usize, default_val: T) -> Self 
    {
        Mat {
            internal_vec: vec![default_val; rows * cols],
            rows: rows,
            cols: cols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Result<&T, MatError> {
        if row >= self.rows || col >= self.cols {
            return Err(MatError::OutOfBounds)
        }
        return Ok(&self.internal_vec[col + self.cols * row]);
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) -> Result<(), MatError> {
        if row >= self.rows || col >= self.cols {
            return Err(MatError::OutOfBounds)
        }
        self.internal_vec[col + self.cols * row] = val;
        Ok(())
    }

    pub fn get_slice(&self, row: usize, col_start: usize, col_end: usize) -> Result<Vec<T>, MatError> {
        if row >= self.rows || col_start >= self.cols || col_end >= self.cols {
            return Err(MatError::OutOfBounds)
        }
        let start = row * self.cols + col_start;
        let end = row * self.cols + col_end;
        Ok(self.internal_vec[start..(end + 1)].to_vec())
    }

    pub fn get_width(&self) -> usize  { self.cols }
    pub fn get_height(&self) -> usize { self.rows }
}

impl<T: fmt::Display> fmt::Display for Mat<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.internal_vec[col + self.cols * row])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


pub fn get_input_filename(curr_filename: &str) -> &str {
    let filename = Path::new(curr_filename).file_stem().unwrap().to_str().unwrap();
    let day_num = &filename[filename.len() - 2..];
    return day_num
}
