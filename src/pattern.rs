#[derive(Clone)]
pub struct Pattern<const PATTERN_SIZE: usize> {
  pub in_pattern: [bool; PATTERN_SIZE],
  pub out_pattern: [bool; PATTERN_SIZE],
}


