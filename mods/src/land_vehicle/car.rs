enum Make{ 
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
    num_wheels:u8,
    max_passengers : u8,
    top_speed:u8,
    make : Make,
    year : u16 // change to u32 around year 65534 C.E.
}
