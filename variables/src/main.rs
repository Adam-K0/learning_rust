use rand::Rng;
use std::io;

const PI: f32 = 3.14159265;
const AVOGADRO_NUM: f64 = 6.022e23;
const MONKEYS: [char; 5] = ['🐒', '🐵', '🙈', '🙉', '🙊'];

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    println!("Circle's circumference over diameter: {PI}");
    println!("Molecules of O2 in 32 grams of Oxygen gas: {AVOGADRO_NUM}");

    // shadowing
    let word1 = "foo";
    {
        let word1 = word1.len();
        println!("length of word1: {word1}");
    }
    println!("word1 is {word1}");
    
    // data types
    let adam_fav_emoji: char = '🙈';
    println!("Adam's favorite emoji is {adam_fav_emoji}");
    
    print_random_monkey();
    let monkey: char = return_random_monkey();
    println!("Random monkey two: {}", monkey);

    let mut input = String::new();
    println!("Enter a number for FizzBuzz:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 =  input.trim().parse().expect("Please type a number");
    fizzbuzz(input);
}

fn print_random_monkey(){
    let random_index = rand::thread_rng().gen_range(0..=4); // produces integer in range [0,4]
    let monkey  = MONKEYS[random_index];
    println!("Random monkey: {monkey}");
}
fn return_random_monkey()-> char{
    let random_index = rand::thread_rng().gen_range(0..=4); // produces integer in range [0,4]
    let monkey  = MONKEYS[random_index];
    return monkey
}

fn fizzbuzz(num: u32){
    for i in 1..=num{        
        let fizz: bool = if i % 3 == 0 {true} else {false};
        let buzz: bool = if i % 5 == 0 {true} else {false};

        if fizz && buzz{
            println!("fizzbuzz");
        }
        else if fizz{
            println!("fizz");
        }
        else if buzz{
            println!("buzz");
        }
        else{
            println!("{i}");
        }
    }
}
