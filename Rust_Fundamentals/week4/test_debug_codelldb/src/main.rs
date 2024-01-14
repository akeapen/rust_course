use std::io;
// this example is useful application of `while` because it allows to continue 
// asking for input until the user types a specific word (in this case "stop")

fn main() {
    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Type something ('stop' to quit):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input line");
        println!("You typed: {}", input);
    }
    println!("Goodbye!");
}