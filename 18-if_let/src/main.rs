enum Vehicle
    {
        Car,
        Bus,
        Truck,
        Bicycle,
        Scooter,
    }
fn main() 
    {
        println!("Hello, world!");

        let vehicle1 = Vehicle::Car;
        let vehicle2 = Vehicle::Bus;
        let vehicle3 = Vehicle::Truck;

        let vehicles = [vehicle1,vehicle2,vehicle3];

        for i in vehicles
            {
                // it's like match actually
                // but less boilerplate
                // this does not force us to cover all posibilities
                // but it can, if you want
                if let i == Vehicle::Car
                    {
                        println!("{} is Car", i);
                    }
                if let i == Vehicle::Bus
                    {
                        println!("{} is Bus", i);
                    }
                if let i == Vehicle::Truck
                    {
                        println!("{} is Truck", i);
                    }
                else
                    {
                        println!("{} is Another Thing", i);
                    }
            }
    }
