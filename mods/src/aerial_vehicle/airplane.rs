use super::air_utils;

enum UseCase{
    Commercial,
    Private,
    Military,
    Cargo,
    Other
}

struct Airplane{
    name: String,
    usecase: UseCase,
    max_passengers: u16,
    max_altitude_meters: u32,
}
impl Airplane{
    pub fn fly(&self){
        
    }
}
