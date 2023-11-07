extern crate masuda;

use masuda::filter::{Filter, StatComparison, StatFilter};
use masuda::generators::{Game, Method};
use masuda::pokemon::Nature;
use masuda::Profile;
use masuda::Searcher;

fn main() {
    let mut searcher = Searcher::new(Game::Emerald, Method::One, 10000000);

    let profile = Profile::new(10101, 12345);
    let filter = Filter::new(&profile)
        .with_stat(StatFilter::HP(StatComparison::EqualTo(31)))
        .with_stat(StatFilter::Attack(StatComparison::EqualTo(31)))
        .with_stat(StatFilter::Speed(StatComparison::EqualTo(31)))
        .with_nature(Nature::Jolly);

    for result in searcher.search(Some(filter)) {
        println!("{:?}", result);
    }
}
