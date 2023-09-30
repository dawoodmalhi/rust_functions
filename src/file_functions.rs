use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub fn extract_colors_codes_occurences_from_text_file(file_path: &str){
    // Create a path to the desired file
    let path = Path::new(file_path);
    // let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = File::open(&path).unwrap();

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Calculate colors occurences count in the string
    let mut result: HashMap<String, u16> = HashMap::new();
    let mut tmp_line: String;
    for line in contents.split_whitespace() {
        if line.contains("#"){
            tmp_line = line.replace("'", "");
            tmp_line = tmp_line.replace(",", "");
            if result.contains_key(&tmp_line) {
                *result.get_mut(&tmp_line).unwrap() += 1;
            } else {
                result.insert(tmp_line, 1);
            };
        }
    }
    
    // Printing results
    for (key, value) in result {
        println!("{},{}", key, value)
    }
}