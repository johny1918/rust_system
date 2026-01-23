use std::{fs::File, io::{BufRead, BufReader}, path::Path};

// Explicit lifetime annotation
pub fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

// Generic function
pub fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

// Practice string, grep-lite like
pub fn search_string(input_string: &str, search_word: &str) {
    for line in input_string.lines() {
        if line.contains(search_word) {
            println!("Found the word you searched for: {}", search_word);
        }
    }
}

// Automatically increment index
pub fn search_string_v2(input_string: &str, search_word: &str) {
    for (i, line) in input_string.lines().enumerate() {
        if line.contains(search_word) {
            let line_num = i + 1;
            println!("Found the word you searched for: {} at line {}", search_word, line_num);
        }
    }
}

// Read from file via BufReader::lines()
pub fn read_a_file(file: &Path) {
    let f = File::open(file).unwrap();
    let reader = BufReader::new(f);
    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}