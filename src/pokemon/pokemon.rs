use std::fmt;

use std::convert::TryFrom;

use crate::pokemon::{Gender, IndividualValues, Nature, NATURES};

pub struct Pokemon {
    pub pid: u32,
    pub ivs: IndividualValues,
}

impl Pokemon {
    pub fn new(pid: u32, ivs: IndividualValues) -> Self {
        Pokemon { pid: pid, ivs: ivs }
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

    // shininess is determined by the process described here:
    // https://www.smogon.com/ingame/rng/pid_iv_creation#how_shiny
    pub fn get_shininess(&self, tid: u16, sid: u16) -> bool {
        let hid = (self.pid >> 16) as u16; // 16 highest bits
        let lid = (self.pid & 65535) as u16; // 16 lowest bits

        for bit_idx in (3..16).rev() {
            let hid_bit = (hid >> bit_idx) & 1;
            let lid_bit = (lid >> bit_idx) & 1;
            let tid_bit = (tid >> bit_idx) & 1;
            let sid_bit = (sid >> bit_idx) & 1;

            if (hid_bit + lid_bit + tid_bit + sid_bit) % 2 != 0 {
                return false;
            }
        }
        true
    }
}

impl fmt::Debug for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x} {} {:?}", self.pid, self.get_nature(), self.ivs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shininess_true() -> Result<(), String> {
        let pid = 0xB58F0B2Au32;
        let ivs = IndividualValues::new(0, 0, 0, 0, 0, 0);
        let p = Pokemon::new(pid, ivs);

        let tid = 0xA918u16;
        let sid = 0x17BBu16;

        assert_eq!(p.get_shininess(tid, sid), true);
        Ok(())
    }

    #[test]
    fn test_shininess_false() -> Result<(), String> {
        let pid = 0xC58F0B2Au32;
        let ivs = IndividualValues::new(0, 0, 0, 0, 0, 0);
        let p = Pokemon::new(pid, ivs);

        let tid = 0xA918u16;
        let sid = 0x17BBu16;

        assert_eq!(p.get_shininess(tid, sid), false);
        Ok(())
    }
}
