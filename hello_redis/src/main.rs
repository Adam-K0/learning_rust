use redis::Commands;

fn main() {
		
	// Init & connect to Redis server
	let mut r = match redis::Client::open("redis://127.0.0.1:6379/") {
		Ok(client) => {
			match client.get_connection() {
				Ok(conn) => conn,
				Err(e) => {
					println!("Failed to connect to Redis: {e}");
					return;
				}
			}
		},
		Err(e) => {
			println!("Failed to create Redis client: {e}");
			return;
		}
	};

	// insert key foo, with value bar
	if let Ok(res) = r.set("foo", "bar") {
		let res: String = res;
		println!("{res}");    // >>> OK
	} else {
		println!("Error setting foo");
	}
	// grab the val associated with foo
	match r.get("foo") {
		Ok(res) => {
			let res: String = res;
			println!("{res}");   // >>> bar
		},
		Err(e) => {
			println!("Error getting foo: {e}");
			return;
		}
	};

	// insert Redis hash(similar a Rust struct)
	let hash_fields = [
		("force_constant", 100.0),
		("relaxed_len", 0.2), // in meters
		("min_len", 0.1),
	];

	if let Ok(res) = r.hset_multiple("spring:1", &hash_fields) {
		let res: String = res;
		println!("{res}"); // >>> OK
	} else {
		println!("Error setting spring:1");
	}

	// grab associated values for given key
	match r.hget("spring:1", "force_constant") {
		Ok(res) => {
			let res: f32 = res;
			println!("{res} N/m"); // >>> 100 N/m
		},
		Err(e) => {
			println!("Error getting spring:1 force_constant: {e}");
			return;
		}   
	}

	match r.hget("spring:1", "min_len") {
		Ok(res) => {
			let mut res: f32 = res;
			println!("{res}"); // >>> 0.1 
		},
		Err(e) => {
			println!("Error getting spring:1 price: {e}");
			return;
		}   
	}

	match r.hgetall("spring:1") {
		Ok(res) => {
			let res: Vec<(String, f32)> = res;
			
			for (key, value) in res {
				println!("{key}: {value}");
			}
		},
		Err(e) => {
			println!("Error getting spring:1: {e}");
			return;
		}   
	}

}
