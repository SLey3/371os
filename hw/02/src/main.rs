use std::env;
use std::fs;
use std::io::{self, Read};
use std::process::exit;

#[derive(Default)]
struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
    max_line_length: usize,
}

fn count_content(content: &str) -> Counts {
    let mut counts = Counts::default();

    counts.bytes = content.as_bytes().len(); // exact byte count

    for line in content.lines() {
        counts.lines += 1;
        counts.words += line.split_whitespace().count();
        counts.chars += line.chars().count();
        counts.max_line_length = counts.max_line_length.max(line.chars().count());
    }

    counts
}

fn print_counts(counts: &Counts, filename: &str, flags: &Vec<char>) {
    // GNU wc order: l, w, m, c, L
    let order = ['l', 'w', 'm', 'c', 'L'];
    for &flag in &order {
        if flags.contains(&flag) {
            match flag {
                'l' => print!("{}\t", counts.lines),
                'w' => print!("{}\t", counts.words),
                'm' => print!("{}\t", counts.chars),
                'c' => print!("{}\t", counts.bytes),
                'L' => print!("{}\t", counts.max_line_length),
                _ => {
                    eprintln!("Unknown flag: {}. Use --help.", flag);
                    exit(1)
                }
            }
        }
    }
    println!("{}", filename);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut flags: Vec<char> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    for arg in &args[1..] {
        if arg.starts_with("--") {
            match arg.as_str() {
                "--lines" => if !flags.contains(&'l') { flags.push('l') },
                "--words" => if !flags.contains(&'w') { flags.push('w') },
                "--bytes" => if !flags.contains(&'c') { flags.push('c') },
                "--chars" => if !flags.contains(&'m') { flags.push('m') },
                "--max-line-length" => if !flags.contains(&'L') { flags.push('L') },
                "--help" => {
                    println!("\
                        Usage: wc [OPTION]... [FILE]...
                        Print newline, word, and byte counts for each FILE, and a total line if
                        more than one FILE is specified.  A word is a non-zero-length sequence of
                        characters delimited by white space.

                        With no FILE, read standard input.

                        Options:
                        -c, --bytes            print the byte counts
                        -m, --chars            print the character counts
                        -l, --lines            print the newline counts
                        -L, --max-line-length  print the maximum display width
                        -w, --words            print the word counts
                            --help             display this help and exit
                            --version          output version information and exit
                    ");
                    exit(0);
                }
                "--version" => {
                    println!("my_wc 0.1.0");
                    exit(0);
                }
                _ => {
                    eprintln!("Unknown flag: {}. Use --help.", arg);
                    exit(1);
                }
            }
        } else if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg.chars().skip(1) {
                match ch {
                    'l' | 'w' | 'c' | 'm' | 'L' => {
                        if !flags.contains(&ch) {
                            flags.push(ch);
                        }
                    }
                    _ => {
                        eprintln!("Unknown flag: -{}. Use --help.", ch);
                        exit(1);
                    }
                }
            }
        } else {
            files.push(arg.clone());
        }
    }

    if flags.is_empty() {
        flags = vec!['l', 'w', 'c']; // default
    }

    if files.is_empty() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        let counts = count_content(&input);
        print_counts(&counts, "", &flags);
    } else {
        let mut total_counts = Counts::default();
        for filename in &files {
            let content = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("my_wc: cannot open '{}'", filename);
                String::new()
            });

            let counts = count_content(&content);
            print_counts(&counts, filename, &flags);

            total_counts.lines += counts.lines;
            total_counts.words += counts.words;
            total_counts.bytes += counts.bytes;
            total_counts.chars += counts.chars;
            total_counts.max_line_length =
                total_counts.max_line_length.max(counts.max_line_length);
        }

        if files.len() > 1 {
            print_counts(&total_counts, "total", &flags);
        }
    }
}
