use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter which fibonacci number to calculate:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 =  input.trim().parse().expect("Please type a number");
    let mut left: u128 = 0;
    let mut right: u128 = 1;
    let mut temp: u128;
    let mut index: u32 = 0;
    while input !=  index {
        temp = left; 
        left = right;
        right = temp + right;
        index += 1;
    }
    println!("{left} at index {index}")
}
