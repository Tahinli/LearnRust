const PI: f64 = 3.14;

// we added derivation for debugging.
// this trait's going to help us for printing in this code
#[derive(Debug)]
struct Ellipse {
    x_radius: f64,
    y_radius: f64,
}

// this implementation helps us to collect related things together
// in this example all things under this will be related to Ellipse struct
// methods gets "self" as a first parameter, this is different from functions
// functions like "circle" no need to get self as a parameter
// we call them related function, they can be used as a constructor
impl Ellipse {
    fn area(&self) -> f64 {
        self.x_radius * self.y_radius * PI
    }
    fn x_radius(&self) -> f64 {
        self.x_radius
    }
    fn y_radius(&self) -> f64 {
        self.y_radius
    }
    fn can_fit(&self, second: &Ellipse) -> bool {
        self.x_radius >= second.x_radius && self.y_radius >= second.y_radius
    }
    fn circle(size: f64) -> Self {
        Self {
            x_radius: (size),
            y_radius: (size),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let ellipse1 = Ellipse {
        x_radius: 2.0,
        y_radius: 3.0,
    };
    let ellipse2 = Ellipse {
        x_radius: 1.0,
        y_radius: 2.0,
    };
    let circle = Ellipse::circle(5.0);
    println!("Ellipse is {:#?}", ellipse1);
    println!("Area of the ellipse is {}", ellipse1.area());
    println!("X = {}", ellipse1.x_radius());
    println!("Y = {}", ellipse1.y_radius());
    println!(
        "Is first be able to hug second: {}",
        ellipse1.can_fit(&ellipse2)
    );
    println!("Are of new circle = {}", circle.area());
}
