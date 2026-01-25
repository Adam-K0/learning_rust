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

use crate::aerial_vehicle::aerial_utils;
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
        aerial_utils::takeoff(false);
    }
    pub fn land(&self){
        aerial_utils::land(false);
    }
}
