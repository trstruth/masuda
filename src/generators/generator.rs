use crate::pokemon::Pokemon;

pub trait Generator {
    /// Four RNG calls are made, two to generate the PID and two to generate the IVs. It can be illustrated as [PID] [PID] [IVs] [IVs].
    fn method_1(&mut self) -> Pokemon;

    /// Five RNG calls are made. The first two are used to generate the PID and the last two are used to generate the IVs. The third RNG call is not used for anything. It can be illustrated as [PID] [PID] [xxxx] [IVs] [IVs].
    fn method_2(&mut self) -> Pokemon;

    /// Five RNG calls are made. The first and second are used to generate the PID and the third and fifth are used to generate the IVs. The fourth RNG call is not used for anything. It can be illustrated as [PID] [PID] [IVs] [xxxx] [IVs].
    fn method_4(&mut self) -> Pokemon;
}

pub enum Method {
    One,
    Two,
    Four,
}

pub enum Game {
    FireRed,
    LeafGreen,
    Emerald,
    Ruby,
    Sapphire,
}
