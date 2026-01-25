mod land_vehicle;
mod aerial_vehicle;
mod submarine;


fn main() {
    use aerial_vehicle::airplane;

    let x15 = airplane::Airplane::new(String::from("NASA X15"), airplane::UseCase::Other, 1,100000); 

    x15.takeoff();

    x15.land();

    use land_vehicle::car;
    let mut dream_car = car::Car{
        num_wheels : 4,
        max_passengers : 2,
        top_speed_kph : 345,
        make: car::Make::Porsche,
        year: 2014
    };
    
    dream_car.year = 2013;
}
