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
        println!("Getting up to {} meters", self.max_altitude_meters); 
    }
    pub fn land(&self){
        crate::aerial_vehicle::aerial_utils::land(false);
    }
}
