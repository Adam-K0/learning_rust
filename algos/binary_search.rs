use std::io;

fn main(){
	let (nums, target) = gather_input();
	println!("Num list = {:?}", nums);
	
	let result = binary_search(&nums, &target);
	if result != usize::MAX{
		println!("Found {target} at {result} index");
	}
	else{
		println!("Didn't find {target} within {nums:?}");
	}
}

fn binary_search(nums: &Vec<i32>, target: &i32)-> usize{

	let mut l: usize = 0;
	let mut r: usize = nums.len()-1;
	let mut m: usize;

	while l <= r{
		m = l + ((r - l) / 2);
		println!("left: {l}, right: {r}, middle {m}");
		println!("{}", m);

		if nums[m] < *target{ // ignore the left half
			l = m+1;
		}
		else if nums[m] > *target{ // ignore the right half
			r = m-1;
		}
		else{
			return m;	
		}
	}
	return usize::MAX 
}

// gather input from user
fn gather_input()-> (Vec<i32>, i32){
	let mut nums = Vec::new();
	let mut input = String::new();

	println!("Enter sorted number list (enter after each num & 0 to finish list)");
	loop{
		input.clear();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		let num: i32 = match input.trim().parse() {
			Ok(n) => n,
			Err(_) => {
				println!("Please type a valid number");
				continue;
			}
		};
		if num == 0{
			break;
		}
		nums.push(num);
	}

	println!("Enter number to search for");
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
