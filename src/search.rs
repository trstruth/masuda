use std::fmt;

use crate::generators::{Game, Generator, LinearCongruential, Method};
use crate::pokemon::Pokemon;
use crate::Filter;

pub struct Searcher {
    method: Method,
    rng: Box<dyn Generator>,
    frame_limit: usize,
}

impl Searcher {
    pub fn new(game: Game, method: Method, frame_limit: usize) -> Self {
        let rng = match game {
            Game::FireRed | Game::LeafGreen => Box::new(LinearCongruential::new(0)),
            Game::Emerald => Box::new(LinearCongruential::new(0)),
            Game::Ruby | Game::Sapphire => Box::new(LinearCongruential::new(0x5A0)),
        };

        Self {
            method,
            rng,
            frame_limit,
        }
    }

    fn advance(&mut self) -> Pokemon {
        match self.method {
            Method::One => self.rng.method_1(),
            Method::Two => self.rng.method_2(),
            Method::Four => self.rng.method_4(),
        }
    }

    pub fn search(&mut self, filter: Option<Filter>) -> Vec<SearchResult> {
        let mut results = Vec::new();

        for frame in 0..self.frame_limit {
            let p = self.advance();
            if let Some(filter) = &filter {
                if !filter.matches(&p) {
                    continue;
                }
            }
            results.push(SearchResult { pokemon: p, frame });
        }

        results
    }
}

pub struct SearchResult {
    pub pokemon: Pokemon,
    pub frame: usize,
}

impl fmt::Debug for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "frame {}: {:?}", self.frame, self.pokemon,)
    }
}
