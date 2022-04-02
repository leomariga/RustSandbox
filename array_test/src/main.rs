use std::io;

fn main() {
    let a = [2,5,6,7];

    println!("Please enter an array index.");

    let mut index = String::new();


    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}
