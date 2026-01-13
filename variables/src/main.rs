use rand::Rng;

const PI: f32 = 3.14159265;
const AVOGADRO_NUM: f64 = 6.022e23;
const MONKEYS: [char; 5] = ['🐒', '🐵', '🙈', '🙉', '🙊'];

fn main() {
    // mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    // consts
    println!("Circle's circumference over diameter: {PI}");
    println!("Molecules of O2 in 32 grams of Oxygen gas: {AVOGADRO_NUM}");

    // shadowing
    let word1 = "fizz";
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
    println!("{monkey}");
}

fn print_random_monkey(){
    let monkey = return_random_monkey();
    println!("Random monkey: {monkey}");
}
fn return_random_monkey()-> char{
    let random_index = rand::thread_rng().gen_range(0..=4); // produces integer in range [0,4]
    let monkey  = MONKEYS[random_index];
    monkey
}

