//We can call modules with use to bring them into scope

//Call them seperately
//use organizing_files::gift_first;
//use organizing_files::gift_second;

//Call them together
use organizing_files::{lake, plain};

//Call every export with "glob"
//use organizing_files::*;

fn main() {
    println!("Hello, world!");

    //Call duck from lake scope
    lake::duck::hi_from_duck();

    //Call bunny from plain scope
    plain::bunny::hi_from_bunny();
}
