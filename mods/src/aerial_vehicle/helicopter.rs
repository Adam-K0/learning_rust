
pub struct Helicopter{
    name : String,
    max_passengers: u8,
    num_rotors: u8,
    total_blades: u8,
}


use crate::aerial_vehicle;
impl Helicopter{

    fn new(name: String, max_passengers: u8, num_rotors: u8, total_blades: u8) -> Self{
        Self{
            name : name,
            max_passengers,
            num_rotors: num_rotors,
            total_blades: total_blades
        }
    }

    fn land(&self){
        aerial_vehicle::land(true);
    }
    fn takeoff(&self){
        aerial_vehicle::takeoff(true);
    }
}
