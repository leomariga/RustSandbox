use std::cmp::Ordering;
use std::io;
fn main() {
    // ask user which fibonnacci number it wants to obtain

    println!("Select the nth fibonnacci number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let myindex: u8 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(error) => panic!("Problem converting: {:?}", error),
                };
    println!("my index: {}",myindex);

    get_fibonnacci(myindex);
}

fn get_fibonnacci(i: u8){
    let mut i1 = 1;
    let mut i2 = 1;
    println!("{}-th fibonnacci is {}",1,i1);
    println!("{}-th fibonnacci is {}",2,i2);
    for number in (3..i+1) {
        let mut i3 = i1 + i2;
        i1 = i2;
        i2 = i3;
        println!("{}-th fibonnacci is {}",number,i3);
    }
}
    
