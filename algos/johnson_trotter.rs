use std::io;
use std::collections::HashSet;

fn main(){
	// Gather user input
	let mut input = String::new();

	println!("Positive integer to generate permutation up to(inclusive)");
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read input");
	let input: usize = input.trim().parse().expect("Provide a number");

	// Initializing vars
	let mut num_vec: Vec<usize> = (1..=input).collect();
	let mut direction_vec: Vec<bool> = vec![true; input as usize]; // true for left, false for right

	let mut mobile_index: Option<usize> = Some(num_vec.len()-1);

	let mut perm_set = HashSet::new();

	// Building permutation set
	while mobile_index.is_some() {

		perm_set.insert(num_vec.clone());

		let k = num_vec[mobile_index.unwrap()]; // mobile_index is guaranteed to be some here so unwrap is fine
		
		let index = mobile_index.unwrap(); 

		if direction_vec[index] { // left swap
			num_vec.swap(index, index-1);
			direction_vec.swap(index, index-1);
		} else { // right swap
			num_vec.swap(index, index+1);
			direction_vec.swap(index, index+1);
		}

		// reverse larger elements
		for i in 0..num_vec.len(){
			if num_vec[i] > k{
				direction_vec[i] = !direction_vec[i];
			}
		}

		// println!("mobile index {}", mobile_index.unwrap());
		// println!("{:?}", direction_vec);
		// println!("{:?}", num_vec);
		// println!("");

		// recalculate mobile index
		mobile_index = max_mobile_index(&num_vec, &direction_vec);
	}
	perm_set.insert(num_vec.clone());

	println!("There are {} permutations", perm_set.len());

	println!("{:?}", perm_set);
}

// a mobile_index element is an element that is larger than what it direclty points to
fn max_mobile_index(num_vec: &Vec<usize>, direction_vec: &Vec<bool>) -> Option<usize> {
	let mut max: Option<usize> = None;

	if num_vec[0] > num_vec[1] && !direction_vec[0]{ // check leftmost element 
		max = Some(0);
	}

	for i in 1..=num_vec.len()-2{
		let left_num = num_vec[i-1];
		let cur_num = num_vec[i];
		let right_num = num_vec[i+1];

		if left_num < cur_num && direction_vec[i]{ // check left neighbor
			if cur_num > num_vec[max.unwrap_or(i-1)] {
				max = Some(i);
			}
		}

		if right_num < cur_num && !direction_vec[i]{ // check right neighbor
			if cur_num > num_vec[max.unwrap_or(i+1)] {
				max = Some(i);
			}
		}
	}

	let last_index = num_vec.len()-1;	
	if num_vec[num_vec.len()-1] > num_vec[num_vec.len()-2] && direction_vec[num_vec.len()-1] { // check rightmost element
		if num_vec[num_vec.len()-1] > num_vec[max.unwrap_or(num_vec.len()-2)]{
			max = Some(num_vec.len()-1);
		}	
	}
	max
}

