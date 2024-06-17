fn main() {
    println!("Hello, world!");

    //Integer
    let a: i8 = 127; //(2^7-1)
    let au: u8 = 255; //(2^8-1)
    println!("a={a}\nau={au}");

    let b: i16 = 32767; //(2^15-1)
    let bu: u16 = 65535; //(2^16-1)
    println!("b={b}\nbu={bu}");

    let c: i32 = 2147483647; //(2^31-1)
    let cu: u32 = 4294967295; //(2^32-1)
    println!("c={c}\ncu={cu}");

    let d: i64 = 9223372036854775807; //(2^63-1)
    let du: u64 = 18446744073709551615; //(2^64-1)
    println!("d={d}\ndu={du}");

    let e: i128 = 170141183460469231731687303715884105727; //(2^127-1)
    let eu: u128 = 340282366920938463463374607431768211455; //(2^128-1)
    println!("e={e}\neu={eu}");

    let f: isize = d as isize; //Based on Architecture of CPU = i64 for me
    let fu: usize = du as usize; //Also Explicit Conversion
    println!("f={f}\nfu={fu}");

    //Float
    let g: f32 = 0.123456789123456789; //7-precision  I think
    let h: f64 = 0.123456789123456789; //17-precision I think
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
    let tup = ("Tahinli", 13, 3.14);
    println!("Tuple = {}-{}-{}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup; //Destructuring
    println!("X-Y-Z = {x}-{y}-{z}");

    //Array
    let array_1 = [0, 1, 2, 3, 4];
    //I haven't learn finite loop in Rust sorry.
    println!(
        "array_1[0]={}\narray_1[1]={}\narray_1[2]={}\narray_1[3]={}\narray_1[4]={}",
        array_1[0], array_1[1], array_1[2], array_1[3], array_1[4]
    );

    let array_2 = [13; 5];
    println!(
        "array_2[0]={}\narray_2[1]={}\narray_2[2]={}\narray_2[3]={}\narray_2[4]={}",
        array_2[0], array_2[1], array_2[2], array_2[3], array_2[4]
    );

    let array_3: [f64; 2] = [0.1, 2.3];
    println!("array_3[0]={}\narray_3[1]={}", array_3[0], array_3[1]);
}
