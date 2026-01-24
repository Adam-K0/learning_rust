// shared aerial vehicle functions

pub fn takeoff(straight: bool){
    if straight{
        println!(r"/ \");
        println!( " | ");
    }
    else{
        println!("   ->");
        println!("  /  ");
        println!("_-   ");
    }
}

pub fn land(straight: bool){
    if straight{
        println!( " | ");
        println!(r"\_/");
    }
    else{
        println!( "-   ");
        println!(r" \  ");
        println!( "  -_");
    }
}
