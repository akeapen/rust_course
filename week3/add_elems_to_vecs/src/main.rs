fn add_elem_both_ends(vec: &mut Vec<i32>, elem: i32) -> Vec<i32> {
    vec.insert(0, elem);
    vec.push(elem);
    return vec.to_vec();
}

fn extend_vec(mut vec: Vec<i32>, app: &[i32]) -> Vec<i32> {
    vec.extend(app);
    return vec.to_vec();
}

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    //println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    //println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8] 

    let v_a = add_elem_both_ends(&mut v, 69);
    println!("{:?}", v_a); // Output: [9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

    let v_b = (0..8).collect::<Vec<i32>>();
    let v_c = extend_vec(v_b, &[10, 11, 12]);
    println!("{:?}", v_c);
}
