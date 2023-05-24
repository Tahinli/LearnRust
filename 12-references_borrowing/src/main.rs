fn main() 
    {
        println!("Hello, world!");

        //We can not have another reference, if we have a mutable one
            let mut s1 = String::from("Safe");
            let r1 = &s1;
            let r2 = &s1;
            //If an unmutable reference will be used, we can not have mutable one 
                //let r3 = &mut s2;
                //println!("References = {}-{}-{}", r1, r2, r3);
            println!("References = {}-{}", r1, r2);
            //r1 and r2 aren't going to be used, so we are free to have mutable reference
            let r3 = &mut s1;
            //One mutable reference will be used, we can not have another
                //let r4 = &mut s2;
                //println!("New References = {}-{}", r3, r4);
            println!("New Mutable Reference = {}", r3);
            //r3 isn't going to use anymore, we are free again to have another mutable reference
            let r4 = &mut s1;
            println!("New Mutable Reference Again = {}", r4);

        let mut s2 = String::from("Traveler");
        //No need to copy same values, or return anything
        //Because we do not give owenership, just borrowing
            borrow1(&s2);
            borrow2(&mut s2);
            println!("s2 = {}", s2);
        //Because of unallocation after scope, this reference would be refered unallocated place
        //Rust checks this, while compling to save us from runtime error
            //let dangle_reference = dangling();
    }
fn borrow1(string_reference : &String)
    {
        println!("Length of String is = {}", string_reference.len());
    }
fn borrow2(string_reference : &mut String)
    {
        string_reference.push_str(" Code");
    }
/*fn dangling() -> &String
    {
        let mut some_string = String::from("Are we dangle ?");
        &some_string
    }*/