fn main() 
    {
        println!("Hello, world!");
        
        let n = 5;
        let s1 = String::from("Learning Ownership");
        take_ownership(s1, n);//s1 is moved n is copied
        //println!("{}", s1); It's gone
        println!("Not Transferred, Just Copied = {}", n);
        
        let s2 = String::from("Be Right Back");
        let s3 = take_then_give_back_ownership(s2);
        //println!("s2 = {}", s2); s2 is long gone
        println!("s3 = {}", s3);

    }

fn take_ownership(our_string:String, our_number:i32)
    {
        //ownership is transferred for string
        println!("I got your String = {}", our_string);
        println!("I got your Number = {}", our_number);
    }   //calls "drop" for string, so our_string is gone now

fn take_then_give_back_ownership(our_string:String) -> String
    {
        println!("Now String is Here = {}", our_string);
        our_string
    }
