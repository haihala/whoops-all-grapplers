pub struct FrameData {
    pub active_start: usize,
    pub recovery_start: usize,
    pub recovered: usize,
}
impl FrameData {
    pub fn new(startup: usize, active: usize, recovery: usize) -> Self {
        Self {
            active_start: startup,
            recovery_start: startup + active,
            recovered: startup + active + recovery,
        }
    }
}
