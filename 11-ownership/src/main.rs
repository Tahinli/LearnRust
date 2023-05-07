fn main() 
    {
        println!("Hello, world!");
        
        let n = 5;
        let s = String::from("Learning Ownership");
        take_ownership(s, n);
        //println!("{}", s); It's gone
        println!("Not Transferred, Just Copied = {}", n);
    
    }

fn take_ownership(our_string:String, our_number:i32)
    {
        //ownership is transferred for string
        println!("I got your String = {}", our_string);
        println!("I got your Number = {}", our_number);
    }   //calls drop for string, so our_string is gone now
