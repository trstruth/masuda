extern crate masuda;

use masuda::generators::{Game, Method};
use masuda::Filter;
use masuda::Profile;
use masuda::Searcher;

fn main() {
    let mut searcher = Searcher::new(Game::Emerald, Method::One, 100000);

    let profile = Profile::new(10101, 12345);
    let filter = Filter::new(&profile).shiny();

    for result in searcher.search(Some(filter)) {
        println!("{:?}", result);
    }
}
