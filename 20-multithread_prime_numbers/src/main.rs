use std::{
    io, thread,
    time::{Duration, Instant},
};
fn get_input() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Expected Valid Number");
            }
        };
    }
}
fn prime(start: u32, finish: u32) {
    // if start > finish {
    //     println!("None");
    //     return;
    // }

    // if finish < 2 {
    //     println!("None");
    //     return;
    // }

    // println!("\t2");
    // if finish == 2 {
    //     return;
    // }

    // if start < 3 {
    //     start = 3;
    // }

    // if start % 2 == 0 {
    //     start += 1;
    // }
    //let mut b;

    for i in (start..=finish).step_by(2) {
        //b = true;
        let square_root = (i as f64).sqrt() as u32;
        for j in (3..=square_root).step_by(2) {
            if i % j == 0 {
                //println!("{} is even", i);
                //b = false;
                break;
            }
        }
        // if b == true {
        //     println!("\t{}", i);
        // }
    }
}

async fn prime_async(start: u32, finish: u32) {
    // if start > finish {
    //     println!("None");
    //     return;
    // }

    // if finish < 2 {
    //     println!("None");
    //     return;
    // }

    // println!("\t2");
    // if finish == 2 {
    //     return;
    // }

    // if start < 3 {
    //     start = 3;
    // }

    // if start % 2 == 0 {
    //     start += 1;
    // }
    //let mut b;

    for i in (start..=finish).step_by(2) {
        //b = true;
        let square_root = (i as f64).sqrt() as u32;
        for j in (3..=square_root).step_by(2) {
            if i % j == 0 {
                //println!("{} is even", i);
                //b = false;
                break;
            }
        }
        // if b == true {
        //     println!("\t{}", i);
        // }
    }
}
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // println!("Start Point for Prime = ↓");
    // let start = get_input();

    // println!("End Point for Prime = ↓");
    // let finish = get_input();

    println!("How Many Thread Do You Want = ↓");
    let thread_count = get_input();

    let start = 3;
    let finish = 10000000;

    let mut time_values_normal_thread = vec![];
    let mut time_values_tokio_thread = vec![];
    for i in 1..thread_count + 1 {
        let thread_count = i;
        let mut threads = vec![];
        let math_magic = (finish - start) / thread_count;
        let instant = Instant::now();
        for i in 0..thread_count {
            let thread = thread::spawn(move || prime(math_magic * i, math_magic * (i + 1) - 1));
            threads.push(thread);
        }
        for thread in threads {
            thread.join().unwrap();
        }
        let time_elapsed = instant.elapsed();
        time_values_normal_thread.push(time_elapsed);
        println!("Elapsed: {:#?} with {} Normal Thread(s)", time_elapsed, i);
    }

    for i in 1..thread_count + 1 {
        let thread_count = i;
        let mut threads = vec![];
        let math_magic = (finish - start) / thread_count;
        let instant = Instant::now();
        for i in 0..thread_count {
            let thread = tokio::spawn(prime_async(math_magic * i, math_magic * (i + 1) - 1));
            threads.push(thread);
        }
        for _ in threads {
            tokio::join!();
        }
        let time_elapsed = instant.elapsed();
        time_values_tokio_thread.push(time_elapsed);
        println!("Elapsed: {:#?} with {} Tokio Thread(s)", time_elapsed, i);
    }

    let mut normal_bench = (Duration::MAX, 0);
    for i in 0..time_values_normal_thread.len() {
        if normal_bench.0 > time_values_normal_thread[i] {
            normal_bench.0 = time_values_normal_thread[i];
            normal_bench.1 = i + 1;
        }
    }

    let mut tokio_bench = (Duration::MAX, 0);
    for i in 0..time_values_tokio_thread.len() {
        if tokio_bench.0 > time_values_tokio_thread[i] {
            tokio_bench.0 = time_values_tokio_thread[i];
            tokio_bench.1 = i + 1;
        }
    }

    println!(
        "\n\nNormal Thread | Bench Results: Min time {:#?} with {} thread",
        normal_bench.0, normal_bench.1
    );

    println!(
        "\n\nTokio Thread | Bench Results: Min time {:#?} with {} thread",
        tokio_bench.0, tokio_bench.1
    );
}
