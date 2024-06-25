use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Define the path to the input file
    let input_file = "input_structure.txt";

    // Read the input text from the file
    let input_text = read_lines(input_file)?;

    // Create the directory and file structure
    create_structure(input_text)?;

    println!("Directory and file structure created successfully.");
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn create_structure(lines: Vec<String>) -> io::Result<()> {
    let mut current_path: Vec<String> = Vec::new();

    for line in lines {
        let indent_level = line.len() - line.trim_start().len();
        let element = line.trim();

        if element.starts_with('/') {
            // It's a directory
            if indent_level == 0 {
                current_path = vec![element.trim_start_matches('/').to_string()];
            } else {
                current_path.truncate(indent_level / 4);
                current_path.push(element.trim_start_matches('/').to_string());
            }
            fs::create_dir_all(Path::new(&current_path.join("/")))?;
        } else {
            // It's a file
            let file_path = current_path.join("/") + "/" + element;
            File::create(Path::new(&file_path))?;
        }
    }

    Ok(())
}
