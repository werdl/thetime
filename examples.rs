// Run this file to generate the doc_comments.txt file
// which is to be put into the README.md file

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Write;

fn main() -> io::Result<()> {
    let file_path = "src/lib.rs";
    let output_path = "doc_comments.txt";

    let input_file = File::open(file_path)?;
    let mut output_file = File::create(output_path)?;

    let mut in_code_block = false;

    for line in io::BufReader::new(input_file).lines() {
        let line = line?;
        let trimmed_line = line.trim();

        if trimmed_line.contains("```") {
            in_code_block = !in_code_block;
            writeln!(output_file, "{}", trimmed_line.replace("/// ", ""))?;

            if !in_code_block {
                writeln!(output_file)?;
            }
        } else if in_code_block && trimmed_line.starts_with("///") {
            // Assuming you want only line doc comments within code blocks
            writeln!(output_file, "{}", trimmed_line.replace("/// ", ""))?;
        }
    }

    Ok(())
}
