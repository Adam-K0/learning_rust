use super::aerial_utils;

pub struct Helicopter{
    name : String,
    max_passengers: u8,
    num_rotors: u8,
    total_blades: u8,
}
impl Helicopter{
    fn land(&self){
        aerial_utils::land(true);
    }
}
