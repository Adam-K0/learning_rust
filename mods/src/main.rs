mod land_vehicle;
mod aerial_vehicle;
mod submarine;


fn main() {
    use aerial_vehicle::airplane::Airplane;
    use aerial_vehicle::airplane::UseCase;

    let x15 = Airplane{name : String::from("NASA X-15"), usecase: UseCase::Other, max_passengers : 1, max_altitude_meters: 100000};

    x15.takeoff();

    use aerial_vehicle::aerial_utils::land;
    x15.land(false);
}
