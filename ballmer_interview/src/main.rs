use std::{thread, time::Duration};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	let runs = 100_000;
	let drop = 1;

	let mut total: i32 = 0;
	let mut lowest = total;
	let mut greatest = total;

	for _ in 0..runs{
		let mut run_total: i32 = 5;
		let secret_num = rand::thread_rng().gen_range(1..=100); 
		println!("Secret number is {secret_num}"); 

		let mut lower = 1;
		let mut upper = 101;

		loop{
			let guess = ((upper-lower) / 2_i32 ) + lower;
			// thread::sleep(Duration::from_millis(100));

			println!("\t Guessed: {guess}");

			match guess.cmp(&secret_num) {
				Ordering::Less => {
					lower += (upper-lower)/2_i32;
					println!("\t new lower {lower} \n");
					// println!("Too small")
				}
 				Ordering::Greater => {
					upper -= (upper-lower)/2_i32;
					println!("\t new upper {upper} \n");
					// println!("Too big"),
				}
				Ordering::Equal => {
					total += run_total;
					if total < lowest{
						lowest = total;
					}
					if total > greatest{
						greatest = total;
					}
					println!("\t Earned/lost {run_total} \n");
					break;
				}
			}
			run_total -= drop;
		}
	}

	println!("After {runs} runs, earned/lost {total}");
	println!("Average run earned/lost {}", total as f32/runs as f32);
	println!("Lowest earnings/loses {lowest} and greatest {greatest}");
}
