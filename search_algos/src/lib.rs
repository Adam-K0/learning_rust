use rand::Rng;

fn sequential(list: &[i32], target: i32) -> Option<&i32>{
    for i in list{
        if *i == target{
           return Some(i);
        } 
    }
    None
}

/* W.I.P
fn progressive_guessing(list: &[i32], target: i32) -> Option<&i32>{
    let mut indicesGuessed = Vec::with_capacity(list.len());
    let mut guess = rand::thread_rng().gen_range(0..list.len());
    let mut numGuesses = 0;

    while numGuesses < list.len(){
        new_valid_guess(&indicesGuessed, guess);
        if list[guess] == target{
            return Some(&list[guess]);
        }
        else{
            indicesGuess.push(guess);
            numGuesses += 1;
        }
    }
    return None;

    fn new_valid_guess(&prevGuessed: Vec, guess: i32){
        if prevGuessed.contains(guess){
            guess = rand::thread_rng().gen_range(0..list.len());
        }
    }
}
*/

// Testing
#[cfg(test)]
mod tests {
    use super::*;

	const NUMS: [i32; 4] = [5,12,55,19];
	const TARGET_NUM: i32 = 55;

    #[test]
    fn seq_search(){
		let mut local_nums= NUMS.to_vec();
		let mut local_target_num = TARGET_NUM;

        assert_eq!(sequential(&local_nums, local_target_num).unwrap(), &local_target_num); 

        local_nums.push(123);
        assert_eq!(sequential(&local_nums, local_target_num).unwrap(), &local_nums[2]); 

        local_nums.push(55); // repeat val
        assert_eq!(*sequential(&local_nums, local_target_num).unwrap(), local_target_num);
        
        local_target_num = 1; // won't find
        assert_eq!(sequential(&local_nums, local_target_num), None);
    }

    #[test]
    #[should_panic]
    fn seq_search_fail(){
        let mut local_nums= NUMS.to_vec();
        let mut local_target_num = TARGET_NUM; 
        
        sequential(&local_nums, local_target_num-1).expect("Value wasn't found");
    }
}