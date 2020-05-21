use std::convert::TryFrom;

/// Represents a linear congruential generator https://en.wikipedia.org/wiki/Linear_congruential_generator
pub struct LinearCongruential {
    seed: u32,
    multiplier: u32,
    increment: u32,
}

impl LinearCongruential {
    /// Constructs a new lcrng instance with the given initial seed
    pub fn new(initial_seed: u32) -> Self {
        LinearCongruential {
            seed: initial_seed,
            multiplier: 0x41C64E6Du32,
            increment: 0x6073u32,
        }
    }

    /// Advances the rng, outputting a new u32 and setting the seed of the instance
    pub fn next_u32(&mut self) -> u32 {
        let result = (self.seed as u64 * self.multiplier as u64) + self.increment as u64;
        let shifted_result = (result << 32) >> 32; // consider using modulo instead of double shift
        let result_u32 = u32::try_from(shifted_result).unwrap();
        self.seed = result_u32;
        result_u32
    }

    /// Advances the rng, outputting the the first 16 bits of the newly generated seed
    pub fn next_u16(&mut self) -> u16 {
        let new_u32 = self.next_u32();
        let shifted_u32 = new_u32 >> 16;
        u16::try_from(shifted_u32).unwrap()
    }

    /// Advances the rng by 2, as a PID is created from 2 rng calls.
    ///
    /// Appending these two 16-bit numbers together results in a 32-bit number, which becomes the PID of the Pokemon.
    /// The second random number becomes the first 16 bits of the PID, and the first random number becomes the second 16 bits.
    pub fn generate_pid(&mut self) -> u32 {
        let n1 = self.next_u16();
        let n2 = self.next_u16();

        let pid: u32 = ((n2 as u32) << 16) + n1 as u32;

        pid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcrng_new() -> Result<(), String> {
        let lcrng = LinearCongruential::new(0);
        assert_eq!(lcrng.seed, 0);
        Ok(())
    }

    #[test]
    fn test_lcrng_next_32() -> Result<(), String> {
        let mut lcrng = LinearCongruential::new(0x1A56B091u32);
        let new_u32 = lcrng.next_u32();
        assert_eq!(new_u32, 0x01DBBC30u32);
        assert_eq!(lcrng.seed, 0x01DBBC30u32);
        Ok(())
    }

    #[test]
    fn test_lcrng_next_16() -> Result<(), String> {
        let mut lcrng = LinearCongruential::new(0x1A56B091u32);
        let new_u16 = lcrng.next_u16();
        assert_eq!(new_u16, 0x01DBu16);
        Ok(())
    }

    #[test]
    fn test_lcrng_next_16_sequence() -> Result<(), String> {
        let mut lcrng = LinearCongruential::new(0x1A56B091u32);
        let expected_numbers: [u16; 6] = [
            0x01DBu16,
            0x7B06u16,
            0x5233u16,
            0xE470u16,
            0x5CC4u16,
            0x36BBu16,
        ];

        for expected_number in expected_numbers.iter() {
            assert_eq!(*expected_number, lcrng.next_u16());
        }

        Ok(())
    }

    #[test]
    fn test_generate_pid() -> Result<(), String> {
        let mut lcrng = LinearCongruential::new(0x1A56B091u32);
        let pid = lcrng.generate_pid();
        assert_eq!(pid, 0x7B0601DBu32);
        Ok(())
    }
}
