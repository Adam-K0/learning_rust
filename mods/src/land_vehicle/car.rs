pub enum Make{ 
    Ford, 
    Infiniti, 
    Audi, 
    Peugeot, 
    Mercedes, 
    Honda, 
    Toyota, 
    Porsche
}

pub struct Car{
    pub num_wheels:u8,
    pub max_passengers : u8,
    pub top_speed_kph:u16,
    pub make : Make,
    pub year : u16 // change to u32 around year 65534 C.E.
}

