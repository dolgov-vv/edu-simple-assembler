
pub type RegIndex = usize;
pub type Label = String;
pub type IP = usize;

#[derive(Debug, Copy, Clone)]
pub struct Flags {
    pub is_zero: bool,
    pub is_sign: bool
}