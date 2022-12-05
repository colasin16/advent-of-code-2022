use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;
use std::path::Path;

pub fn read_file_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file_path: &Path = Path::new(filename);
    let file = File::open(file_path)?;
    Ok(BufRead::lines(BufReader::new(file)))
}
