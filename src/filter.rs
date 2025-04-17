use crate::pokemon::{IndividualValues, Nature, Pokemon};
use crate::Profile;

use std::collections::HashSet;

pub struct Filter {
    tid: u16,
    sid: u16,
    shiny: bool,
    stat_filters: StatFilters,
    nature_filter: NatureFilter,
}

impl Filter {
    pub fn new(profile: &Profile) -> Self {
        Self {
            shiny: false,
            tid: profile.tid,
            sid: profile.sid,
            stat_filters: StatFilters::new(),
            nature_filter: NatureFilter::Any,
        }
    }

    pub fn shiny(mut self) -> Self {
        self.shiny = true;
        self
    }

    pub fn with_stat(mut self, sf: StatFilter) -> Self {
        self.stat_filters.set(sf);
        self
    }

    pub fn with_nature(mut self, nature: Nature) -> Self {
        match self.nature_filter {
            NatureFilter::Any => {
                let mut natures = HashSet::new();
                natures.insert(nature);
                self.nature_filter = NatureFilter::Natures(natures);
            }
            NatureFilter::Natures(ref mut natures) => {
                natures.insert(nature);
            }
        }
        self
    }

    pub fn with_natures(mut self, natures: Vec<Nature>) -> Self {
        for nature in natures {
            self = self.with_nature(nature)
        }

        self
    }

    pub fn matches(&self, p: &Pokemon) -> bool {
        if !self.stat_filters.matches(&p.ivs) {
            return false;
        }

        if !self.nature_filter.matches(&p.get_nature()) {
            return false;
        }

        if self.shiny && !p.get_shininess(self.tid, self.sid) {
            return false;
        }
        true
    }
}

pub enum NatureFilter {
    Any,
    Natures(HashSet<Nature>),
}

impl NatureFilter {
    fn matches(&self, nature: &Nature) -> bool {
        match self {
            NatureFilter::Any => true,
            NatureFilter::Natures(natures) => natures.contains(nature),
        }
    }
}

pub struct StatFilters {
    hp: StatFilter,
    atk: StatFilter,
    def: StatFilter,
    spa: StatFilter,
    spd: StatFilter,
    spe: StatFilter,
}

impl StatFilters {
    fn new() -> Self {
        StatFilters {
            hp: StatFilter::HP(StatComparison::Any),
            atk: StatFilter::Attack(StatComparison::Any),
            def: StatFilter::Defense(StatComparison::Any),
            spa: StatFilter::SpecialAttack(StatComparison::Any),
            spd: StatFilter::SpecialDefense(StatComparison::Any),
            spe: StatFilter::Speed(StatComparison::Any),
        }
    }

    fn set(&mut self, sf: StatFilter) {
        match sf {
            StatFilter::HP(_) => self.hp = sf,
            StatFilter::Attack(_) => self.atk = sf,
            StatFilter::Defense(_) => self.def = sf,
            StatFilter::SpecialAttack(_) => self.spa = sf,
            StatFilter::SpecialDefense(_) => self.spd = sf,
            StatFilter::Speed(_) => self.spe = sf,
        }
    }

    fn matches(&self, ivs: &IndividualValues) -> bool {
        self.hp.matches(ivs.hp)
            && self.atk.matches(ivs.atk)
            && self.def.matches(ivs.def)
            && self.spa.matches(ivs.spa)
            && self.spd.matches(ivs.spd)
            && self.spe.matches(ivs.spe)
    }
}

#[derive(Copy, Clone)]
pub enum StatFilter {
    HP(StatComparison),
    Attack(StatComparison),
    Defense(StatComparison),
    SpecialAttack(StatComparison),
    SpecialDefense(StatComparison),
    Speed(StatComparison),
}

impl StatFilter {
    fn matches(&self, iv: u8) -> bool {
        match self {
            StatFilter::HP(sc) => sc.matches(iv),
            StatFilter::Attack(sc) => sc.matches(iv),
            StatFilter::Defense(sc) => sc.matches(iv),
            StatFilter::SpecialAttack(sc) => sc.matches(iv),
            StatFilter::SpecialDefense(sc) => sc.matches(iv),
            StatFilter::Speed(sc) => sc.matches(iv),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum StatComparison {
    Any,
    EqualTo(u8),
    GreaterThan(u8),
    LessThan(u8),
}

impl StatComparison {
    fn matches(&self, iv: u8) -> bool {
        match self {
            StatComparison::Any => true,
            StatComparison::EqualTo(n) => iv == *n,
            StatComparison::GreaterThan(n) => iv > *n,
            StatComparison::LessThan(n) => iv < *n,
        }
    }
}
