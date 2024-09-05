use std::{
    error::Error,
    io::{self, Read},
    net::TcpListener,
};

// Normally main functions return nothing in Rust
// but we're allowed to return numeric values
// it's convention from C and Rust wanted to fit for this also.
// If your process works successfully you just return 0
// else you return something other then zero.
// You can specify these returns by Termination trait
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // This is a way to handle a case/error, because you are handling every situation
    // I like this because I can clearly decide what to do
    // but I can accept that it's too much verbose for some cases,
    // especially if you want to just do things for error/none cases
    // so we have "if let" for this
    match read_from_connection("127.0.0.1:1") {
        //if everything is good it will be Ok state,
        Ok(buffer) => println!("{:#?}", buffer),
        //if we got an error it will be Err state
        Err(err_val) => eprintln!("Error: Read from socket | {}", err_val),
    }

    let input = -4.0;
    // This "if let" allows us to apply action for each case separately
    // This is good if you want to just take an action for errors or vice versa
    if let None = square_rooter(input) {
        // This part will work because
        // I wrote my function to return None if it gets negative number
        println!("{} is not positive number", input)
    }

    let input = input * -1.0;
    if let Some(result) = square_rooter(input) {
        // This part will work because
        // I wrote my function to return Some(square root of input) if it gets positive number
        println!("{} is the result !", result)
    }

    // This question mark says it will just return early if it reaches None or Err case
    // It eliminates boilerplates
    // This must return code earlier if you don't start the code with higher privileges
    // because port 1 is belongs to operating system
    // You will see an error with an output code
    // We can return this output code because we defined that main function can do this
    // (in function header we gave output type)
    read_from_connection("127.0.0.1:1")?;
    Ok(())
}

// This function is just a scenario for returning Result
// Basically it's trying to listen a socket and get data from it
// We have a lot of '?' in this function to eliminate boilerplate
fn read_from_connection(addr: &str) -> Result<Vec<u8>, io::Error> {
    // This tries binding to socket
    let tcp_listener = TcpListener::bind(addr)?;
    // This tries accepting connection from socket
    let mut tcp_stream = tcp_listener.accept()?.0;
    let mut buffer = vec![];
    // This tries read value from socket
    tcp_stream.read_to_end(&mut buffer)?;
    Ok(buffer)
}

// This function is an example about another boilerplate eliminator.
// It just calculates square root of a number
//
// It's basically this:
//
// if input > 0.0 {
//     Some(input.sqrt())
// } else {
//     None
// }
//
// Actual function is not that performant as above
// but it can be useful
fn square_rooter(input: f64) -> Option<f64> {
    (input > 0.0).then(|| input.sqrt())
}
