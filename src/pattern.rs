pub trait Pattern<const PATTERN_SIZE: usize> {}

#[derive(Clone)]
pub struct Pattern3to3<const PATTERN_SIZE: usize> {
  pub in_pattern: [bool; PATTERN_SIZE],
  pub out_pattern: [bool; PATTERN_SIZE],
}

#[derive(Clone)]
pub struct Pattern3to1<const PATTERN_SIZE: usize> {
  pub in_pattern: [bool; PATTERN_SIZE],
  pub out_pattern: bool,
}

impl<const PATTERN_SIZE: usize> Pattern<PATTERN_SIZE> for Pattern3to3<PATTERN_SIZE> {}
impl<const PATTERN_SIZE: usize> Pattern<PATTERN_SIZE> for Pattern3to1<PATTERN_SIZE> {}
