// input expects the pivot to be in the 0th position
// returns the index of the split
fn hoare_partition(mut array: Vec<u32>) -> Vec<u32> {
	let pivot: u32 = array[0];		
	let mut i: usize = 0;
	let mut j: usize = array.len()-1; 

	while i<j {

		while array[i] < pivot{ // leftmost element smaller than pivot
			i += 1; }

		while array[j] > pivot{ // rightmost element greater than pivot
			j -= 1; }

		array.swap(i,j);
		}

	array.swap(i,j); // undo the last swap, where i>=j
	array.swap(0, j); // place pivot in appropriate slot

	return array
}

fn main(){

	let array = vec![3,6,13,2,10,0];
	println!("Before partition {:?}", array);
	let result  = hoare_partition(array) ;
	assert_eq!(result, [3, 2, 0, 13, 10, 6]);
	println!("After partition {:?}", result);
}
