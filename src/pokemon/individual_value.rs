use std::fmt;

#[derive(PartialEq, Clone, Default)]
pub struct IndividualValues {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spa: u8,
    pub spd: u8,
    pub spe: u8,
}

impl IndividualValues {
    pub fn new(hp: u8, atk: u8, def: u8, spa: u8, spd: u8, spe: u8) -> Self {
        IndividualValues {
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
        }
    }

    /// Constructs a new IndividualValues struct from the bits of 2 random numbers
    ///
    /// ```mono
    /// First Random Number:  x|xxxxx|xxxxx|xxxxx
    ///                       -|DefIV|AtkIV|HP IV
    /// Second Random Number: x|xxxxx|xxxxx|xxxxx
    ///                       -|SpDIV|SpAIV|SpeIV
    /// ```
    ///
    /// As each IV is expressed as 5 bits, the range of possible values is 0-31
    /// For example, given the subsequent two random numbers 0x5233 and 0xE470 as above, we would have:
    /// First Random Number = 0x5233 = 0|10100|10001|10011. Hence, the Defense IV would be 10100 = 20, the Attack IV would be 10001 = 17 and the HP IV would be 10011 = 19.
    /// Second Random Number = 0xE470 = 1|11001|00011|10000. Hence, the Special Defense IV would be 11001 = 25, the Special Attack IV would be 00011 = 3 and the Speed IV would be 10000 = 16.
    /// Thus, our Pokemon would have the IVs 19/17/20/3/25/16
    ///
    /// ```
    /// # use masuda::pokemon::IndividualValues;
    /// let n1 = 0x5233u16;
    /// let n2 = 0xE470u16;
    /// let expected_ivs = IndividualValues::new(19, 17, 20, 3, 25, 16);
    /// let calculated_ivs = IndividualValues::new_from_numbers(n1, n2);
    /// assert_eq!(expected_ivs, calculated_ivs);
    /// ```
    pub fn new_from_numbers(n1: u16, n2: u16) -> Self {
        let hp = (n1 & 0x1Fu16) as u8;
        let atk = ((n1 & 0x3E0u16) >> 5) as u8;
        let def = ((n1 & 0x7C00u16) >> 10) as u8;
        let spe = (n2 & 0x1Fu16) as u8;
        let spa = ((n2 & 0x3E0u16) >> 5) as u8;
        let spd = ((n2 & 0x7C00u16) >> 10) as u8;

        IndividualValues {
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
        }
    }
}

impl fmt::Debug for IndividualValues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}/{}/{}",
            self.hp, self.atk, self.def, self.spa, self.spd, self.spe
        )
    }
}
