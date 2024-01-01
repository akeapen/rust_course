use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // add the ability to pass in any file path to avoid hard-coding the path in the program
    let args: Vec<String> = env::args().collect();
    // let mut args: Vec<String> = env::args().collect();
    // accept a user defined file name as a command line argument
    let indicated_file = String::from("/Cargo.toml");
    let fname = args[0].clone() + &indicated_file;    
    
    // The first argument is the path that was used to call the program.
    // println!("My path is {:?}.", args[0]); //&filename );
    println!("My path is {:?}.", fname); 
    
    let file = File::open("non_existent_file.txt");
    
    
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
}