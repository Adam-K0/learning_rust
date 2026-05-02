fn lomuto(mut vector: Vec<i32>) -> Vec<i32> {
	let p = vector[0];
	let mut s = 0;

	for i in 1..=vector.len(){
		if vecotr[i] < p{
			s += 1;
			vector.swap(s,i);
		}
	}
	vector.swap(0,s);

	return vector
}

fn main(){
	lomuto();
}
