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

    #[test]
    fn seq_search(){
        let nums = vec![5,12,55,19];
        let targetNum = 55;

        assert_eq!(*sequential(&nums, targetNum).unwrap(), targetNum);
    }
}
