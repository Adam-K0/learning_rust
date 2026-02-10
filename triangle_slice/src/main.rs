fn main() {
    let aster = String::from("********");
    let aster_len = aster.len();

    println!("aster: {aster}");
    println!("aster's length: {aster_len}");

    for i in (0..=aster_len){
        println!("{}", &aster[0..i]);
    }
    for i in (0..aster_len).rev(){
        println!("{}", &aster[0..i]);
    }
}
