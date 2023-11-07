# masuda <img src="https://archive-media-1.nyafuu.org/vp/image/1538/51/1538515003451.gif" width="100" title="dragonite">

[Junichi Masuda](https://en.wikipedia.org/wiki/Junichi_Masuda) is a Japanese video game composer, director, designer, producer, singer, programmer and trombonist, best known for his work in the Pokémon franchise. He is a member of the Game Freak board of directors, and has been employed at the company since 1989 when he founded it alongside Satoshi Tajiri and Ken Sugimori.

### Overview
In the pokemon games, many of the interactions in the game are decided by the output of a _pseudo-random number generator_, or pRNG.  For example, when the player encounters a pokemon, its stats are determined by the state of the pRNG.

In the generation 3 games (R/S/E/Fr/Lg), the game displays roughly 60 frames/second, and the state of the pRNG is said to be "advanced" every frame.  This means if the player knows
* the initial seed passed to the pRNG
* the mechanics behind how the pRNG is advanced
* how the numbers outputted by the pRNG are converted into pokemon stats

Then the user can find a frame which will produce a desireable pokemon, wait for the amount of time before the desired frame occurs, then encounter the desired pokemon.

masuda is a library written in rust which implements the behavior of the random number generator used in the pokemon games.  Specifically, it provides an implementation of a [linear congruential generator](https://en.wikipedia.org/wiki/Linear_congruential_generator) which produces a sequence of pseudo random numbers.

### Usage
Try running the examples to see it in action

`cargo run --example ten_frames` generates 10 frames and the pokemon generated on those frames

`cargo run --example is_shiny` finds the shiny frames in the first 100000 frames for the given tid/sid and displays them

most usages of this lib will probably look something like this:
```rust
// create an searcher instance with game, method, and max frames
let mut searcher = Searcher::new(Game::Emerald, Method::One, 1000000);
// create a profile with a tid and an sid
let profile = Profile::new(10101, 12345);
// create a filter for the attributes you care about
// in this example, let's look for a 31/31/x/x/x/31 Jolly spread
let filter = Filter::new(&profile)
        .with_stat(StatFilter::HP(StatComparison::EqualTo(31)))
        .with_stat(StatFilter::Attack(StatComparison::EqualTo(31)))
        .with_stat(StatFilter::Speed(StatComparison::EqualTo(31)))
        .with_nature(Nature::Jolly); 

for result in searcher.search(Some(Filter)) {
    println("{:?}", result);
}
```
