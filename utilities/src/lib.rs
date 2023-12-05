use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("Input file path should exist")
}

// fn read_input(path: &str) -> Result<String, Box<dyn Error>> {
//     fs::read_to_string(path)
//         .map_err(|e| format!("Error reading input file: {}", e).into())
// }