fn neg_index<T>(arr: &[T], idx: isize) -> Option<&T> {
  let n = arr.len() as isize;

  // Convert negative: -1 -> n-1, -2 -> n-2, ...
  let i = if idx < 0 { n + idx } else { idx };

  if i < 0 || i >= n {
    None
  } else {
    Some(&arr[i as usize])
  }
}
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

  fn left_border_wrap(&self, neg_overlap: isize) -> [bool; PATTERN_SIZE] {
    let mut ret = [false; PATTERN_SIZE];
    let mut ctr = 0;
    for index in neg_overlap..neg_overlap + (PATTERN_SIZE as isize) {
      ret[ctr] = *neg_index(&self.world, index).unwrap();
      ctr += 1
    }
    return ret;
  }

  fn right_border_wrap(&self, pos_overlap: usize) -> [bool; PATTERN_SIZE] {
    let mut ret = [false; PATTERN_SIZE];
    let mut ctr = 0;
    // Ignore items until this pos
    let left_most = self.world.iter().skip((self.world.len() - 1) - pos_overlap);
    let mut ctr = 0;
    for elem in left_most {
      ret[ctr] = *elem;
      ctr += 1
    }
    for idx in 0..(-(1 + pos_overlap as isize) + PATTERN_SIZE as isize) as usize {
      ret[ctr] = self.world[idx];
      ctr += 1
    }
    ret
  }

  pub fn neg_overlap(i: usize) -> isize {
    i as isize - ((PATTERN_SIZE / 2) as isize)
  }

  // In src/world.rs:
  pub fn pos_overlap(&self, i: usize) -> usize {
    // 1. Calculate the index immediately *after* the chunk ends
    let end_index_exclusive = i + PATTERN_SIZE / 2 + 1;

    // 2. If this exclusive end index is beyond the WORLD_SIZE, calculate the difference (overlap)
    if end_index_exclusive > WORLD_SIZE {
      end_index_exclusive - WORLD_SIZE
    } else {
      0
    }
  }

  pub fn get_wrapping_chunks_at_pos_i(&mut self, i: usize) -> [bool; PATTERN_SIZE] {
    // We are tying to an index that is so close to the left border that we wrap around
    let neg_overlap = Self::neg_overlap(i);
    if neg_overlap < 0 {
      return self.left_border_wrap(neg_overlap);
    }
    let pos_overlap = self.pos_overlap(i);
    if pos_overlap > 0 {
      // We moved past the right border
      return self.right_border_wrap(pos_overlap);
    }
    let ret = &self.world[i - PATTERN_SIZE / 2..i + PATTERN_SIZE / 2 + 1];
    assert!(ret.len() == PATTERN_SIZE);
    return ret.try_into().unwrap();
  }

  pub fn get_chunks_of_size_cyclig(&mut self) -> Option<[bool; PATTERN_SIZE]> {
    let max_start_index = WORLD_SIZE - 1;
    if self.current_slice_ptr > max_start_index {
      // Reset the pointer and fall through to the non-wrapping logic below
      self.current_slice_ptr = 0;
      return None;
    }
    let ret = self.get_wrapping_chunks_at_pos_i(self.current_slice_ptr);
    self.current_slice_ptr += 1;
    Some(ret)
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
      let c2 = w.get_chunks_of_size();
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
      assert!(*c1 == [false, true, false, false]);
    }
    {
      let c2: &[bool; 4] = w.get_chunks_of_size().unwrap();
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

  #[test]
  fn neg_overlap() {
    let r1 = World::<5, 3>::neg_overlap(0);
    assert_eq!(r1, -1);

    let r2 = World::<5, 5>::neg_overlap(0);
    assert_eq!(r2, -2);

    let r3 = World::<5, 3>::neg_overlap(1);
    assert_eq!(r3, 0);

    let r4 = World::<5, 5>::neg_overlap(1);
    assert_eq!(r4, -1);
  }

  #[test]
  fn pos_overlap_size_three() {
    let w: World<5, 3> = World::new([true, false, false, false, true]);
    let r1 = w.pos_overlap(4);
    assert_eq!(r1, 1);

    let r1 = w.pos_overlap(3);
    assert_eq!(r1, 0);
  }

  #[test]
  fn pos_overlap_size_five() {
    let w: World<5, 5> = World::new([true, false, false, false, true]);
    let r1 = w.pos_overlap(4);
    assert_eq!(r1, 2);

    let r1 = w.pos_overlap(3);
    assert_eq!(r1, 1);

    let r1 = w.pos_overlap(2);
    assert_eq!(r1, 0);
  }

  #[test]
  fn left_border_wrap_one() {
    let w: World<5, 3> = World::new([true, false, false, false, true]);
    let r1 = w.left_border_wrap(-1);
    assert_eq!(r1, [true, true, false]);

    let r2 = w.left_border_wrap(-2);
    assert_eq!(r2, [false, true, true]);
  }

  #[test]
  fn left_border_wrap_two() {
    let w: World<5, 5> = World::new([true, false, false, false, true]);
    let r1 = w.left_border_wrap(-1);
    assert_eq!(r1, [true, true, false, false, false]);

    let w: World<5, 5> = World::new([true, false, false, false, true]);
    let r1 = w.left_border_wrap(-2);
    assert_eq!(r1, [false, true, true, false, false]);
  }

  #[test]
  fn right_border_wrap_one() {
    let w: World<5, 3> = World::new([false, false, false, false, true]);
    let r1 = w.right_border_wrap(1);
    assert_eq!(r1, [false, true, false]);

    let w: World<5, 5> = World::new([false, false, false, false, true]);
    let r2 = w.right_border_wrap(2);
    assert_eq!(r2, [false, false, true, false, false]);
  }
  #[test]
  fn right_border_wrap_failing_one() {
    let w: World<5, 5> = World::new([true, false, false, false, true]);
    let r3 = w.right_border_wrap(2);
    assert_eq!(r3, [false, false, true, true, false]);
  }

  #[test]
  fn test_get_chunk_no_wrap() {
    let mut world: World<10, 3> = World::new([
      false, true, false, true, false, true, false, true, false, true,
    ]);

    // Choose index i=4. Chunk should be world[3..6], which is [F, T, F]
    let i = 4;

    let chunk = world.get_wrapping_chunks_at_pos_i(i);
    assert_eq!(
      chunk,
      [true, false, true],
      "Should extract chunk without wrapping."
    );

    let mut world: World<10, 3> = World::new([
      false, true, false, true, false, true, false, true, false, true,
    ]);
    // Choose index i=8. End index i+1 = 9. Should NOT wrap.
    let i = 8;
    let chunk = world.get_wrapping_chunks_at_pos_i(i);
    assert_eq!(
      chunk,
      [true, false, true],
      "Should NOT wrap, should extract [F, T, F]."
    );
  }

  /// Test the 'Left Wrap' case where the index is too close to the start.
  #[test]
  fn test_get_chunk_left_wrap() {
    let mut world: World<10, 3> = World::new([
      false, true, false, true, false, true, false, true, false, true,
    ]);

    // Choose index i=0. Start index i-1 = -1. Should trigger left_border_wrap.
    let i = 0;

    let chunk = world.get_wrapping_chunks_at_pos_i(i);
    // Should match the mocked return from left_border_wrap
    assert_eq!(
      chunk,
      [true, false, true],
      "Should trigger and return the left wrap result."
    );

    // Choose index i=1. Start index i-1 = 0. Should NOT wrap.
    let i = 1;
    let chunk = world.get_wrapping_chunks_at_pos_i(i);
    // Expected chunk: [world[0], world[1], world[2]] => [T, F, T]
    assert_eq!(
      chunk,
      [false, true, false],
      "Should NOT wrap, should extract [T, F, T]."
    );
  }

  #[test]
  fn test_get_chunk_right_wrap() {
    let mut world: World<10, 3> = World::new([
      false, true, false, true, false, true, false, true, false, true,
    ]);

    let i = 9;
    let chunk = world.get_wrapping_chunks_at_pos_i(i);
    assert_eq!(
      chunk,
      [false, true, false],
      "Should trigger and return the left wrap result."
    );
  }
  #[test]
  fn test_get_chunks_cycling() {
    // WORLD: [T, F, F, F, T] (Size 5)
    // PATTERN: 3. Chunk is [i-1, i, i+1]
    let mut w: World<5, 3> = World::new([true, false, false, false, true]);
    let mut results: Vec<[bool; 3]> = Vec::new();

    // Collect all chunks in one cycle (i=0 to i=4)
    loop {
      if let Some(chunk) = w.get_chunks_of_size_cyclig() {
        results.push(chunk);
      } else {
        break;
      }
    }

    // Expected chunks:
    // i=0 (L-Wrap): [4, 0, 1] -> [T, T, F]
    // i=1 (No Wrap): [0, 1, 2] -> [T, F, F]
    // i=2 (No Wrap): [1, 2, 3] -> [F, F, F]
    // i=3 (No Wrap): [2, 3, 4] -> [F, F, T]
    // i=4 (R-Wrap): [3, 4, 0] -> [F, T, T]
    let expected = vec![
      [true, true, false],
      [true, false, false],
      [false, false, false],
      [false, false, true],
      [false, true, true],
    ];

    // 1. Check that all 5 chunks were returned
    assert_eq!(
      results.len(),
      5,
      "Should return exactly WORLD_SIZE chunks before resetting."
    );

    // 2. Check the content of all chunks
    assert_eq!(
      results, expected,
      "The chunks should be correctly extracted and wrapped in order."
    );

    // 3. Check that the pointer was reset
    assert_eq!(
      w.current_slice_ptr, 0,
      "The pointer should be reset to 0 after the cycle ends."
    );

    // 4. Check that the next call starts the cycle again
    assert_eq!(
      w.get_chunks_of_size_cyclig(),
      Some([true, true, false]),
      "The function should immediately start the next cycle (i=0) after reset."
    );
  }
}
