fn main() {
    let s1 = String::from("long string is long");
	let s2 = String::from("xyz");

	let result = longest(&s1, &s2);

	println!("The longest string is {result}");
}

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
	x
}
