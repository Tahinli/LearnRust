fn main() 
    {
        println!("Hello, world!");

        
        //Keeps data in heap
        //Information about data are in stack
        //Because data can be changed in runtime

                //"Move" like shallow copy but invalidates original
            let s1 = String::from("Some Data");
            let s2 = s1;
            //println!("s1 = {}", s1); s1 isn't valid
            println!("s2 = {}", s2);
                //"Clone" like deep copy
            let s3 = String::from("Another Data");
            let s4 = s3.clone(); //Data in heap will be copied
            println!("s3 = {}", s3);
            println!("s4 = {}", s4);

        //"Copy" all data are in stack
        //Because size is known while compiling
            let x = 5;
            let y = x;
            println!("x = {}", x);
            println!("y = {}", y);
    }