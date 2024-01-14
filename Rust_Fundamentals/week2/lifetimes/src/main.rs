// https://learn.microsoft.com/en-us/training/modules/rust-memory-management/4-exercise-lifetimes
/* In this exercise, you'll reimplement the copy_and_return function so that it returns a reference 
to the value inserted in the vector. Leave the main function as it is. You'll know your task is 
complete when the code runs and all the assertions pass.*/


// TODO: modify only this function.
// fn copy_and_return(vector: &mut Vec<String>, value: &str) {
//     vector.push(String::from(value));
// }
fn copy_and_return<'a> (vector: &'a mut Vec<String>, value: &'a str) -> &'a str {
    vector.push(String::from(value));
    vector.last().unwrap()
    // vector.get(vector.len() - 1).unwrap()

}

fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}