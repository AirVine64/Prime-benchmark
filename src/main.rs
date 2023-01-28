//Crates and modules
use std::time::{Instant, Duration};

fn main() {
    //Variables
    let now_global = Instant::now();
    let mut times: Vec<Duration> = Vec::new();
    let mut percentage: f32;

    //Benchmark loop
    for i in 0..50 {
        //Variables
        let now = Instant::now();
        let mut primes: Vec<i32> = Vec::new();
        let mut z: i32;

        //Main part
        for j in (1..100000).step_by(2) {
            z = 1;
            for k in 2..j+1 {
                if j % k == 0 {
                    z = z + 1;
                    if z == 3 {
                        break;
                    }
                }
            }
            if z == 2 {
                primes.push(j);
            }
        }
        let elapsed = now.elapsed();
        times.push(elapsed);

        //Progress bar
        print!("\x1B[2J\x1B[1;1H");
        print!("[");
        for _l in 0..i+1 {
            print!("#");
        }
        for _l in 0..50-i {
            print!("_");
        }
        print!("]");
        percentage = 100.0 * ((i+1) as f32/(i+1+50-i) as f32);
        println!(" - {:.2}%", percentage);
    }

    //Average calculation
    let mut sum: u128 = 0;
    let mut a: u128 = 0;
    for i in &times {
        a = a + 1;
        sum = sum + i.as_micros();
    }
    sum = sum / a;
    
    //Unit conversion
    let mut sum_f: f64 = 0.0;
    let mut unit = "μs";
    if sum > 1000000 {
        sum_f = sum as f64 / 1000000.0;
        unit = "s";
    }
    else if sum > 1000 {
        sum_f = sum as f64 / 1000.0;
        unit = "ms";
    }

    //Result printout
    let elapsed_global = now_global.elapsed();
    print!("\x1B[2J\x1B[1;1H");
    println!("Average time: {}{}", sum_f, unit);
    println!("Overall time: {:.2?}", elapsed_global);
}
