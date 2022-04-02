fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let a = five();
    println!("The value of x is: {}", a);
    let b = plus_one(6);
    println!("The value of x is: {}", b);

}

fn another_function(x:i32, unit_label:char) {
    println!("Another function. the number is {} and the char is {}", x, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}