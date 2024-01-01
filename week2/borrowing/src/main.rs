fn own_vec(mut vector: Vec<i32>) {
    vector.push(10);
    println!("{:?}", vector);
}

fn borrow_vec(vector: &Vec<i32>) {
    let new_vector = vector.clone();
    // vector.push(10);
    // new_vector.push(10);
    println!("new_vector {:?}", new_vector);
}

// fn own_integer(x: i32) {
//     x + 1;
// }

fn borrow_integer(x: &i32) -> i32 {
    let x_new = x + 1;
    // x + 1;
    x_new
}

fn own_string(s: String) {
    println!("{}", s);
}

fn borrow_string(s: &String, sn: String) {
    let new_string = s.clone() + sn.as_str();
    println!("{}", new_string);
}


// Borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function 
// or another part of your program without actually transferring ownership of the variable. 
// When you borrow a variable, you're essentially saying 
// "I want to use this variable for a little while, but I promise I won't modify it."
fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_string = String::from("Hello, world!");

    // // this compiles no problem!
    // own_integer(my_int);
    // println!("{}", my_int);
    borrow_integer(&my_int);

    borrow_string(&my_string, " ...all is well!".to_string());
    own_string(my_string); // take ownership of my_string
    // this is using my_string which has also moved and is invalid
    //println!("{:?}", my_string); // this will not compile!

    borrow_vec(&my_vec);
    own_vec(my_vec);
    // but this is using my_vec which was borrowed (moved) and yet is now invalid
    //println!("{:?}", my_vec); // this will not compile!
}

// Borrowing is a key concept in Rust because it allows you to write code that is both safe and efficient. 
// By lending ownership of a variable instead of transferring it, Rust ensures that only 
// one part of your program can modify the variable at a time, which helps prevent 
// bugs and makes it easier to reason about your code.