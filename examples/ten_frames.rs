extern crate masuda;

use masuda::generators::LinearCongruential;

fn main() {
    // create a new Linear Congruential Generator with a seed of 0
    let mut rng = LinearCongruential::new(0);

    // generate 10 frames using method 1, and print them
    for frame in 0..10 {
        let p = rng.method_1();
        println!("frame {}: {:?}", frame, p);
    }
}
