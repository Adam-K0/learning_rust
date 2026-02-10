pub enum UseCase{
    Commercial,
    Private,
    Military,
    Cargo,
    Other
}

pub struct Airplane{
    name: String,
    usecase: UseCase,
    max_passengers: u16,
    max_altitude_meters: u32,
}

use super::super::aerial_vehicle;
impl Airplane{

    pub fn new(name: String, usecase: UseCase, max_passengers: u16, max_altitude_meters:u32) -> Self{
        Self{
            name: name,
            usecase: usecase,
            max_passengers: max_passengers,
            max_altitude_meters: max_altitude_meters
        }
    } 

    pub fn takeoff(&self){
        aerial_vehicle::takeoff(false);
    }
    pub fn land(&self){
        aerial_vehicle::land(false);
    }
}
