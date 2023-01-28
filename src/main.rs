//Crates and modules
use std::time::{Instant, Duration};

fn main() {
    //Variables
    let mut primes: Vec<i32>;
    let mut times: Vec<Duration> = Vec::new();
    let mut now;
    let mut z: i32;

    //Benchmark loop
    for _i in 1..10 {
        //Variables
        primes = Vec::new();
        now = Instant::now();

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
    println!("Average time: {}{}", sum_f, unit);
}
