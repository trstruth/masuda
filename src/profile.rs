pub struct Profile {
    pub tid: u16,
    pub sid: u16,
}

impl Profile {
    pub fn new(tid: u16, sid: u16) -> Self {
        Self { tid, sid }
    }
}
