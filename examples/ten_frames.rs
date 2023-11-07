extern crate masuda;

use masuda::generators::{Game, Method};
use masuda::Searcher;

fn main() {
    let mut searcher = Searcher::new(Game::Emerald, Method::One, 10);

    for result in searcher.search(None) {
        println!("{:?}", result);
    }
}
