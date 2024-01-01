
fn get_item(index: usize, vec: &Vec<i32>) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    // let vec = vec![1, 2, 3, 4, 5];

    if vec.is_empty() {
        return println!("The vector is empty!");
    } else {
        println!("The vector is not empty!");
        // Retrieve a value at a specific index
        let value = vec.get(index).unwrap();
        // print the value
        println!("The value at index {} is {:?}", index, value);
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    // let v: Vec<i32> = vec![];

    println!("The vector is: {:?}", &v);

    // Check if the vector is empty
    if v.is_empty() {
        println!("The vector is empty!");
        // break;
        // panic!("The vector is empty!");
    } else {
        get_item(3, &v);

        // Retrieve a value at a specific index
        let third_value = v[2];
        println!("The third value in the vector is: {}", third_value);    

        // Retrieve the last value
        let last_value = v.last().unwrap();
        println!("The last value in the vector is: {}", last_value);

        // Retrieve the first value using pattern matching
        match v.first() {
            Some(first_value) => println!("The first value in the vector is: {}", first_value),
            None => println!("The vector is empty!"),
        }
    }
}
