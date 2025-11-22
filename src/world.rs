#[derive(Clone, Copy, Debug)]
pub struct World<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> {
  pub world: [bool; WORLD_SIZE],
  pub current_slice_ptr: usize,
}

// Repeat the generic parameters <T, const SIZE: usize> here
impl<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> World<WORLD_SIZE, PATTERN_SIZE> {
  // You can use SIZE to create a new instance
  pub fn new(world: [bool; WORLD_SIZE]) -> Self
where {
    // SIZE is accessed directly as a value
    Self {
      world,
      current_slice_ptr: 0,
    }
  }

  pub fn get_chunks_of_size(&mut self) -> Option<&[bool; PATTERN_SIZE]> {
    // Check if the current pointer allows *at least one more* non-wrapping slice of size 1
    // The maximum start index is WORLD_SIZE - PATTERN_SIZE
    let max_start_index = WORLD_SIZE - PATTERN_SIZE;

    if self.current_slice_ptr > max_start_index {
      // Reset the pointer and fall through to the non-wrapping logic below
      self.current_slice_ptr = 0;
      return None;
    }

    // Now we know self.current_slice_ptr is a valid start index (0 to max_start_index)
    let start = self.current_slice_ptr;
    let end = self.current_slice_ptr + PATTERN_SIZE;
    // Advance the pointer for the *next* call
    self.current_slice_ptr += 1;

    // Return the mutable slice, converted to an array reference
    Some(self.world[start..end].try_into().unwrap())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_chunks() {
    let mut w: World<3, 3> = World::new([false, true, false]);
    let c: [bool; 3] = *w.get_chunks_of_size().unwrap();
    assert!(w.world == [false, true, false])
  }

  #[test]
  fn test_get_chunks_staying_at_pos() {
    let mut w: World<4, 4> = World::new([false, true, false, false]);
    {
      let c1: &[bool; 4] = w.get_chunks_of_size().unwrap();
      assert!(*c1 == [false, true, false, false]);
    }
    {
      let c2 = w.get_chunks_of_size() ;
      assert!(c2 == None);
    }
    {
      let c3: &[bool; 4] = w.get_chunks_of_size().unwrap();
      assert!(*c3 == [false, true, false, false]);
    }
  }

  #[test]
  fn test_get_chunks_wrapping() {
    let mut w: World<5, 4> = World::new([false, true, false, false, true]);
    {
      let c1: &[bool; 4] = w.get_chunks_of_size().unwrap();
      dbg!(*c1);
      assert!(*c1 == [false, true, false, false]);
    }
    {
      let c2: &[bool; 4] = w.get_chunks_of_size().unwrap();
      dbg!(*c2);
      assert!(*c2 == [true, false, false, true]);
    }
    {
      let c3 = w.get_chunks_of_size();
      assert!(c3 == None);
    }
  }

  #[test]
  fn test_get_chunks_for_loop() {
    let mut w: World<5, 3> = World::new([false, true, false, false, true]);
    let mut results: Vec<[bool; 3]> = vec![];
    loop {
      if let Some(ding) = w.get_chunks_of_size() {
        results.push(ding.clone());
      } else {
        break;
      }
    }
    assert!(results.len() == 3);
    assert!(results[0] == [false, true, false]);
    assert!(results[1] == [true, false, false]);
    assert!(results[2] == [false, false, true]);
  }
}
