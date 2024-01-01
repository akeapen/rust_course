use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use rand::Rng;

fn generate_random_word() -> String {
    let mut word = String::new();
    for _ in 0..9 {
        let random_char = (b'a' + rand::random::<u8>() % 26) as char;
        word.push(random_char);
    }
    word
}

// write a function to generate a random word from the English dictionary       


fn main() {
    // let file = File::open("non_existent_file.txt");
    let file = File::open("Cargo.toml");
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }

    let write_file = File::create("test.txt");
    match write_file {
        Ok(line) => {
            let mut writer = BufWriter::new(line);
            for _ in 0..10 {
                let word = generate_random_word();
                writeln!(writer, "{}", word).expect("Error writing to file");
            }
        }
        Err(error) => {
            panic!("Error reading line: {}", error)
        }
    }
    
}
