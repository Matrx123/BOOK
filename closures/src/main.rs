use std::thread;
use std::time::Duration;

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_no = 7;
    generate_workout(simulated_intensity, simulated_random_no);

    let x = vec![1, 2, 3];
    let equal_to_x = |z| { z == x };
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_no: u32) {
    let mut cache_results = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today do {:?} puhsups!", cache_results.value(intensity));
        println!("Next do {:?} situps!", cache_results.value(intensity));
    } else {
        if random_no == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {:?} minutes", cache_results.value(intensity));
        }
    }
}
