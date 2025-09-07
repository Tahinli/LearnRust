use std::fmt::{Debug, Display};

// This is a trait for defining shared behaviour.
// This trait only defines a rule about "T" which is a single type of generic.
// Here we defined a default behaviour by giving function body.
// If we didn't give this default, it should be implemented for every data type that shares this trait.
// I said Self needs to implement Debug trait because my default function body is printing in debug mode.
// I said "T" has to implement Display because I wanted to override default later for "Point" struct.
trait Tahinli<T> {
    fn print_data(&self)
    where
        Self: Debug,
        T: Display,
    {
        println!("{:#?}", self)
    }
}

// This is a struct that must validate a type for every field in itself.
#[derive(Debug)]
struct Point<T> {
    x_axis: T,
    y_axis: T,
    z_axis: T,
}

// We implemented our trait here and overrode default behaviour.
// Because I said "T" has to implement Display in both trait definition and here.
// I'm able to print every field of "Point<T>" type by just println!.
impl<T> Tahinli<T> for Point<T> {
    fn print_data(&self)
    where
        T: Display,
    {
        println!("{}", self.x_axis);
        println!("{}", self.y_axis);
        println!("{}", self.z_axis);
    }
}

// This is my another struct that can accept at most 3 type of data types for different fields.
#[derive(Debug)]
struct AnotherPoint<T, U, V> {
    _x_axis: T,
    _y_axis: U,
    _z_axis: V,
}

// I didn't override default implementation so this will follow default one.
impl<T, U, V> Tahinli<T> for AnotherPoint<T, U, V> {}

fn main() {
    println!("Hello, world!");

    // I'm creating a "Point" with only one type of data type for all fields which is i32
    let point = Point {
        x_axis: 0,
        y_axis: 1,
        z_axis: 2,
    };

    // I'm creating "AnotherPoint" with 3 different data type for each different field.
    let another_point = AnotherPoint {
        _x_axis: '0',
        _y_axis: "Tahinli",
        _z_axis: 0.0,
    };

    // I'm able to print them but with different style because of overriding default function body.
    point.print_data();
    another_point.print_data();
}
