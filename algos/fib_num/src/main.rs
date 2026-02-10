use std::io;

fn main() {
	// Gathering & cleaning user input
    let mut input = String::new();
    
    println!("Enter which fibonacci number to calculate:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 =  input.trim().parse().expect("Please type a number");

	// Calculation
    let mut left: u128 = 0;
    let mut right: u128 = 1;
    let mut temp: u128;

    for _ in 0..input {
        temp = left; 
        left = right;
        right = temp + right;
	}

	// Showing result
    println!("{left} at index {input}")
}
