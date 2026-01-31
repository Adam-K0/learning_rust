fn main() {
    fn sequential(list: &[i32], target: i32) -> Option<&i32>{
        for i in list{
            if *i == target{
               return Some(i);
            } 
        }
        None
    }

    fn progressive_guessing(){
    }

    fn rand_guess(){
    }
    
    // testing
    let nums = vec![5,12,55,19];
    let target = 55;

    if let Some(refr) = sequential(&nums, target){
        println!("found val {refr}"); // println will deref refr to print its val 
    }
    else{
        println!("didn't find the val");
    }
}
