#[derive(Debug)]
enum Vehicle {
    Car,
    Bus,
    Truck,
    Bicycle,
    Scooter,
}
fn main() {
    println!("Hello, world!");

    let vehicle1 = Vehicle::Car;
    let vehicle2 = Vehicle::Bus;
    let vehicle3 = Vehicle::Truck;
    let vehicle4 = Vehicle::Bicycle;
    let vehicle5 = Vehicle::Scooter;

    let vehicles = [vehicle1, vehicle2, vehicle3, vehicle4, vehicle5];

    for i in vehicles {
        // it's like match actually
        // but less boilerplate
        // this does not force us to cover all possibilities
        // but it can, if you want
        if let Vehicle::Car = i {
            println!("{:#?} is Car", i);
        } else {
            println!("{:#?} is Another Thing", i);
        }
    }
}
