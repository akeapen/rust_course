use std::io::{self, Write};
// use std::str::FromStr;

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

// fn average(numbers: &[f32]) -> f32 {
//     let mut result = 0.0;
//     for number in numbers {
//         result += number;
//     }
//     result / (numbers.len() as f32)
// }

fn average(numbers: &[f32]) -> Option<f32> {
    if numbers.is_empty() {
        None
    } else {
        let mut result = 0.0;
        for number in numbers {
            result += number;
        }
        Some(result / (numbers.len() as f32))
    }
}

// fn main() {
//     // There are no variadic arguments in Rust
//     let numbers = [1, 2, 3, 4, 5];
//     let result = sum(&numbers);
//     println!("The sum is {}", result);
// }

fn main() {
    let mut input = String::new();

    print!("Enter the size of the vector: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    // `usize` is used as the type for the `size` variable. This means that `size` will hold a  
    // non-negative integer value that represents the size of a collection (like an array or a vector). The 
    // `input.trim().parse().unwrap()` part of the line is converting the user's input from a string to a `usize` value.
    let size: usize = input.trim().parse().unwrap();
    input.clear();

    let mut numbers = Vec::new();
    for _ in 0..size {
        print!("Enter a number: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let number = input.trim().parse().unwrap();
        numbers.push(number);
        input.clear();
    }

    println!("numbers length is {}", numbers.len());

    let result = sum(&numbers);
    println!("The sum is {}", result);
    
    // let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let numbers: Vec<f32> = numbers.iter().map(|&x| x as f32).collect();
    let mean = average(&numbers);
    println!("The average is {:?}", mean);
}