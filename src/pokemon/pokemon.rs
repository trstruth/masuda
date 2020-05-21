use std::convert::TryFrom;

use crate::pokemon::{Gender, IndividualValues, Nature, NATURES};

#[derive(Debug)]
pub struct Pokemon {
    pub pid: u32,
    pub ivs: IndividualValues,
}

impl Pokemon {
    pub fn new(pid: u32, ivs: IndividualValues) -> Self {
        Pokemon{
            pid: pid,
            ivs: ivs,
        }
    }

    /// Ability of a pokemon by determined by the last bit of its pid
    pub fn get_ability(&self) -> u8 {
        (self.pid % 2) as u8
    }

    /// Nature of a pokemon is determined by taking the last 2 digits of the decimal representation of the PID, then computing modulo 25.
    /// The resulting index is then looked up in the NATURES const table.
    pub fn get_nature(&self) -> Nature {
        NATURES[((self.pid % 100) % 25) as usize]
    }
    
    /// Gender of a pokemon is determined by the last bytes of the PID.
    /// A byte can express values between 0-255 inclusive, and the various gender ratios dictate the cutoffs.
    pub fn get_gender_number(&self) -> u8 {
        u8::try_from(self.pid & 0xFFu32).unwrap()
    }

    pub fn get_gender_12_5_f(&self) -> Gender {
        if self.get_gender_number() <= 30 {
            return Gender::Female;
        }
        Gender::Male
    }

    pub fn get_gender_25_f(&self) -> Gender {
        if self.get_gender_number() <= 63 {
            return Gender::Female;
        }
        Gender::Male
    }

    pub fn get_gender_50_f(&self) -> Gender {
        if self.get_gender_number() <= 126 {
            return Gender::Female;
        }
        Gender::Male
    }

    pub fn get_gender_75_f(&self) -> Gender {
        if self.get_gender_number() <= 190 {
            return Gender::Female;
        }
        Gender::Male
    }
}
