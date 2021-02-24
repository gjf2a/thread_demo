use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let total_flips = Arc::new(Mutex::new(0));
    let completed = Arc::new(Mutex::new(0));
    let runs = 8;
    let target_flips = 10;

    for _ in 0..runs {
        let total_flips = total_flips.clone();
        let completed = completed.clone();
        thread::spawn(move || {
            simulate(target_flips, total_flips);
            let mut completed = completed.lock().unwrap();
            *completed += 1;
        });
    }

    loop {
        let completed = completed.lock().unwrap();
        if *completed == runs {
            let total_flips = total_flips.lock().unwrap();
            println!("Final average: {}", *total_flips / *completed);
            break;
        }
    }
}

fn simulate(target_flips: u64, total_flips: Arc<Mutex<u64>>) {
    let mut consecutive = 0;
    let mut iterations = 0;
    while consecutive < target_flips {
        iterations += 1;
        if rand::random() {
            consecutive += 1;
        } else {
            consecutive = 0;
        }
    }
    println!("iterations: {}", iterations);
    let mut total_flips = total_flips.lock().unwrap();
    *total_flips += iterations;
}