pub enum UseCase{
    Commercial,
    Private,
    Military,
    Cargo,
    Other
}

pub struct Airplane{
    pub name: String,
    pub usecase: UseCase,
    pub max_passengers: u16,
    pub max_altitude_meters: u32,
}
impl Airplane{
    pub fn takeoff(&self){
        println!("Getting up to {} meters", self.max_altitude_meters);    
    }
    pub fn land(&self){
        crate::aerial_vehicle::aerial_utils::land(false);
    }
}
