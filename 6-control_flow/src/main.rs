fn main() {
    println!("Hello, world!");

    let x = 3;

    if x<0
        {
            println!("Below Zero");
        }
    else if x==0
        {
            println!("Equals Zero");
        }
    else
        {
            println!("Above Zero");
        }
    let val = true;
    let x = if val {5} else {7};

    println!("X is = {x}"); 
    
    'outer_loop: loop   //Labeling a loop
        {
            println!("Outer Loop");
            'inner_loop: loop 
                {
                    println!("Inner Loop");
                    if x==5
                        {
                           break 'outer_loop; 
                        }
                    else 
                        {
                            break 'inner_loop;
                        }
                }
        }
    let mut y = 0;
    while y==0
        {
            y += 1;
        }
    let q = [0,1,2,3,4];

    for element in q    //For Each
        {
            println!("Element of Array = {element}");
        }

    for element in (0..100).rev()   //Standart For
        {
            println!("Element = {element}");
        }


}
