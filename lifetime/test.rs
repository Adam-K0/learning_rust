fn main() {
    let s1 = String::from("abcd");
	let s2 = String::from("xyz");

	let result = s1.longest(s2);

    println!("The longest string is {result}");
}

fn longest(&self, y: &str) -> &str {
	if self.len() > y.len() { self } else { y }	
}
