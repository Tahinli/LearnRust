fn main() 
    {
        hello();

        let x = {
            let y = 5; //Statement
            y+0 //Expression
        };
        println!("X = {}", x);


        let y = return_value(x);
        println!("Y = {}", y)
    }

fn hello()
    {
        println!("Hello World");
    }

fn return_value(mut x:i32) -> i32
    {
        println!("We're going to return something");
        x = x+1;
        x*x
    }