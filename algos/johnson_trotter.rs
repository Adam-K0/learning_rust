use std::io;

fn main(){
	// Gather user input
	let mut input = String::new();

	println!("Positive integer to generate permutation up to(inclusive)");
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read input");
	let input: u32 = input.trim().parse().expect("Provide a number");

	// Initializing vars
	let mut num_vec: Vec<u32> = (1..=input).collect();
	let mut direction_vec: Vec<bool> = vec![true; input as usize]; // true for left, false for right

	let mut perm_set = HashSet::new();
	perm_set.insert(num_vector.clone());
			
}

fn hasMobile(num_vec: &Vec<u32>, direction_vec: &Vec<u32>) -> bool {

}

fn maxMobile(num_vec: &Vec<u32>, direction_vec: &Vec<bool>) -> Option<u32> {

}