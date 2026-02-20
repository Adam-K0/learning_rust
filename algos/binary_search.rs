use std::io;

fn main(){
	let (nums, target) = gather_input();
	
	let result = binary_search(&nums, &target);
	if result.is_some(){
		println!("Found {target} at index {}", result.unwrap());
	}
	else{
		println!("Didn't find {target} within {nums:?}");
	}
}

fn binary_search(nums: &Vec<i32>, target: &i32)-> Option<usize>{

	let mut l: usize = 0;
	let mut r: usize = nums.len()-1;
	let mut m: usize;

	while l <= r{
		m = l + ((r - l) / 2);

		// uncomment for debugging
		// println!("left: {l}, middle {m}, right {r}");
		// println!("Value at middle: {}", nums[m]);

		if nums[m] < *target{ // ignore the left half
			l = m+1;
		}
		else if nums[m] > *target{ // ignore the right half
			r = m-1;
		}
		else{
			return Some(m);	
		}
	}
	return None 
}

// gather input from user
fn gather_input()-> (Vec<i32>, i32){
	let mut nums = Vec::new();
	let mut input = String::new();

	println!("Enter sorted number list (enter after each num & press Enter on empty line to finish)");
	loop{
		input.clear();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		
		// Check if input is empty (just pressed Enter)
		if input.trim().is_empty(){
			break;
		}
		
		let num: i32 = match input.trim().parse() {
			Ok(n) => n,
			Err(_) => {
				println!("Please type a valid number");
				continue;
			}
		};
		nums.push(num);
	}

	println!("Enter number to search for: ");
	input.clear();
	io::stdin()		
		.read_line(&mut input)
		.expect("Failed to read line");

	let target: i32 = match input.trim().parse() {
		Ok(n) => n,
		Err(_) => {
			println!("Please type a valid number");
			panic!();
		}
	};

	(nums, target)
}
