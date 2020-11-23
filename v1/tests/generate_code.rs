use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[tokio::test]
async fn generate_code() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("tests/generate_code_file.sql") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(_s) = line {

                // str::
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
