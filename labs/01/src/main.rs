use std::fs::read_to_string;
use std::env::args;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}


fn main() {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let filename = args().nth(1).expect("Please Provide Filename");

    let lines = read_lines(&filename);

    for line in lines {
        let word_count = line.split_whitespace().count();
        let byte_count = line.len() + 1;

        total_words += word_count;
        total_bytes += byte_count;
        total_lines += 1;
    }

    println!("\t {total_lines} \t {total_words} \t {total_bytes} \t {filename}");
}
