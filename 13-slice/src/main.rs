fn main() 
    {
        println!("Hello, world!");
        
        //mutable because we will clean it later.
            let mut s = String::from("Yellow Duck");
        //it's not &String, it's literal thanks to slice
            let firts = firts_world(&s[..]);
        println!("{}", firts);
        s.clear();
        //s.clear function has mutable reference to truncate
        //and if we print "first" after this
        //we have to have mutable and immutable references
        //at the same time
        //it's forbidden as we've learned before
            //println!("{}", firts);

        let a = [0,1,2,3,4,5];
        let b = &a[2..4];
        println!("{}-{}", b[0], b[1]);
    }

fn firts_world(s: &str) -> &str
    {   
        //it converts string to byte array
            let bytes = s.as_bytes();

        //iter is iteration we will learn later, even i don't know well
        //enumerate returns index and reference in order as a tuple
            for(i, &item) in bytes.iter().enumerate()
                {
                    //We're trying to catch "space" to sepeterate our string
                    //Book says also we will learn pattern matching later
                        if item == b' '
                            {
                                //when we find "space", we return first slice
                                    return &s[0..i];
                            }
                }
        //If we can not find any space, we return whole string as a literal
            &s[..]
    }
