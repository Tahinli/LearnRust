use std::fmt::Display;

// This is a struct that can take any type into it.
// Only limitation is every field should be same type.
struct Point<T> {
    x_axis: T,
    y_axis: T,
    z_axis: T,
}

impl<T> Point<T> {
    // This creates new point based from values that is given.
    // Their type should be anything and must be same.
    fn new(x_axis: T, y_axis: T, z_axis: T) -> Self {
        Point {
            x_axis,
            y_axis,
            z_axis,
        }
    }
}

// This is a struct that can take any type into it.
// Unlike "Point" its field not have to be same type but it can also be.
struct AnotherPoint<T, U, K> {
    x_axis: T,
    y_axis: U,
    z_axis: K,
}

impl<T, U, K> AnotherPoint<T, U, K> {
    // This creates new point based from values that is given.
    // Their types not have to be same, they can be anything
    fn new(x_axis: T, y_axis: U, z_axis: K) -> Self {
        AnotherPoint {
            x_axis,
            y_axis,
            z_axis,
        }
    }
}

fn main() {
    println!("Hello, world!");

    // It's a bit strange to explain generics without traits.
    // In short trait defines a capability.
    // Generics use traits to match capable things to achieve what generics used for.
    // Instead of defining a lot of functions, structs and enums that do similar things,
    // we define generics to match similar patterns and collapse functions, structs and enums in one.

    // Values of this "Point" variables must have same type.
    let point_1 = Point::new(1, 2, 3);
    let point_2 = Point::new(1.0, 2.0, 3.0);

    // This "AnotherPoint" type is not have limitation like "Point"
    // They can be different types.
    let another_point_1 = AnotherPoint::new(1, 2.0, 'a');
    let another_point_2 = AnotherPoint::new(1, 2, 3);

    // I can call print_point function because it's able to fit our variables
    // Every field has same type in a "Point" type.
    print_point(&point_1.x_axis, &point_1.y_axis, &point_1.z_axis);
    print_point(&point_2.x_axis, &point_2.y_axis, &point_2.z_axis);

    // I can call print_point function because it's able to fit our variables
    // Every field may be different from others in "AnotherPoint" type.
    print_point(
        &another_point_1.x_axis,
        &another_point_1.y_axis,
        &another_point_1.z_axis,
    );
    print_point(
        &another_point_2.x_axis,
        &another_point_2.y_axis,
        &another_point_2.z_axis,
    );
}

// This function takes at most 3 different type of values.
// These values must include Display trait.
// We defined this rule with "where".
// Instead of using where we can say:
// fn print_point<T: Display, U: Display, K: Display>(x_axis: &T, y_axis: &U, z_axis: &K)
fn print_point<T, U, K>(x_axis: &T, y_axis: &U, z_axis: &K)
where
    T: Display,
    U: Display,
    K: Display,
{
    println!("{}", x_axis);
    println!("{}", y_axis);
    println!("{}", z_axis);
}
