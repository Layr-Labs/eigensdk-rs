use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/// Reads a file line by line and returns an iterator over the lines.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
