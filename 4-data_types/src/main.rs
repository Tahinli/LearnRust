fn main() {
    println!("Hello, world!");

    //Integer
        let a:i8 = 127;  //(2^7-1)
        let au:u8 = 255; //(2^8-1)
        println!("a={a}\nau={au}");

        let b:i16 = 32767;  //(2^15-1)
        let bu:u16 = 65535; //(2^16-1)
        println!("b={b}\nbu={bu}");

        let c:i32 = 2147483647;  //(2^31-1)
        let cu:u32 = 4294967295; //(2^32-1)
        println!("c={c}\ncu={cu}");

        let d:i64 = 9223372036854775807;   //(2^63-1)
        let du:u64 = 18446744073709551615; //(2^64-1)
        println!("d={d}\ndu={du}");

        let e:i128 = 170141183460469231731687303715884105727;  //(2^127-1)
        let eu:u128 = 340282366920938463463374607431768211455; //(2^128-1)
        println!("e={e}\neu={eu}");

        let f:isize = d as isize;  //Based on Architechture of CPU = i64 for me
        let fu:usize = du as usize; //Also Explicit Conversion
        println!("f={f}\nfu={fu}");

    //Float
        let g:f32 = 0.123456789123456789; //7-precision  I think
        let h:f64 = 0.123456789123456789; //17-precision I think
        println!("g={g}\nh={h}");

    //Boolean
        let i = true;
        let j = false;
        println!("i={i}\nj={j}");

    //Character
        let k = 'K';
        let l = '🦀';
        println!("k={k}\nl={l}");

    //Tuple
        let tup = ("Tahinli",13,3.14);
        println!("Tuple = {}-{}-{}", tup.0, tup.1, tup.2);
        let (x,y,z) = tup; //Destructuring
        println!("X-Y-Z = {x}-{y}-{z}");

    //Array
        let ar = [0,1,2,3,4];
        //I havent learn finite loop in Rust sorry.
        println!("ar[0]={}\nar[1]={}\nar[2]={}\nar[3]={}\nar[4]={}",ar[0], ar[1], ar[2], ar[3], ar[4]);

        let arr = [13;5];
        println!("arr[0]={}\narr[1]={}\narr[2]={}\narr[3]={}\narr[4]={}",arr[0], arr[1], arr[2], arr[3], arr[4]);

        let arrr : [f64;2] = [0.1,2.3];
        println!("arrr[0]={}\narrr[1]={}", arrr[0], arrr[1]);
}
