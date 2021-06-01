extern crate masuda;

use masuda::generators::LinearCongruential;

fn main() {
    // create a new Linear Congruential Generator with a seed of 0
    let mut rng = LinearCongruential::new(0);

    // use some tid/sid
    let tid = 10101;
    let sid = 12345;

    let mut num_shinies = 0;
    let mut frame = 0;
    while num_shinies < 3 {
        let p = rng.method_1();
        if p.get_shininess(tid, sid) {
            println!("frame {}: {:?} is shiny", frame, p);
            num_shinies += 1;
        }
        frame += 1;
    }
}
