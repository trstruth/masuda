use masuda::{
    filter::{StatComparison, StatFilter},
    generators::{Game, Method},
    Filter, Profile, Searcher,
};

fn main() {
    // create an searcher instance with game, method, and max frames
    let mut searcher = Searcher::new(Game::Emerald, Method::One, 100000000000);
    // create a profile with a tid and an sid
    let profile = Profile::new(10101, 12345);

    let filter = Filter::new(&profile)
        .with_stat(StatFilter::HP(StatComparison::EqualTo(31)))
        .with_stat(StatFilter::Attack(StatComparison::EqualTo(31)))
        .with_stat(StatFilter::Defense(StatComparison::EqualTo(31)))
        .with_stat(StatFilter::SpecialAttack(StatComparison::EqualTo(31)))
        .with_stat(StatFilter::SpecialDefense(StatComparison::EqualTo(31)))
        .with_stat(StatFilter::Speed(StatComparison::EqualTo(31)));

    for result in searcher.search(Some(filter)) {
        println!("{:?}", result);
    }
}
