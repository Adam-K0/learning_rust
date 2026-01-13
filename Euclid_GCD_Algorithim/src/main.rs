use std::io;

fn main(){
    let mut m  = String::new();
    println!("Enter your m value: ");
    io::stdin()
        .read_line(&mut m)
        .expect("Failed to read line");
    let mut m: i32 =  m.trim().parse().expect("Please type a number");

    let mut n = String::new();
    println!("Enter your n value: ");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let mut n: i32 =  n.trim().parse().expect("Please type a number");
    
    let mut placeholder: i32;
    while n != 0 {
        placeholder = m;
        m = n;
        n = placeholder % m;
    }
    println!("{m}");
}

