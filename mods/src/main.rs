mod land_vehicle;
mod aerial_vehicle;
mod submarine;


fn main() {
    use aerial_vehicle::airplane::Airplane;
    use aerial_vehicle::airplane::UseCase;

    let x15 = Airplane{name : String::from("NASA X-15"),
    usecase: UseCase::Other,
    max_passengers : 1,
    max_altitude_meters: 100000};

    x15.takeoff();

    x15.land();

    use land_vehicle::car;
    let mut dreamCar = car::Car{
        num_wheels : 4,
        max_passengers : 2,
        top_speed_kph : 345,
        make: car::Make::Porsche,
        year: 2014
    };
    
    dreamCar.year = 2013;
    println!("The {} Porsche 918 Spyder can get up to {} km per hour!", dreamCar.year, dreamCar.top_speed_kph);
}
