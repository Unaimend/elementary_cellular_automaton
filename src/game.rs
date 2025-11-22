use std::{fs::File, io::Write, path::PathBuf};


use crate::world::*;
use crate::pattern::*;

pub struct Game<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> {
  pub rules: Vec<Pattern<PATTERN_SIZE>>,
  pub state: Vec<World<WORLD_SIZE, PATTERN_SIZE>>,
}

impl<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> Game<WORLD_SIZE, PATTERN_SIZE> {
  pub fn new(rules: Vec<Pattern<PATTERN_SIZE>>) -> Self {
    let beginnning_state = [false; WORLD_SIZE];
    Game {
      rules,
      state: vec![
        World {
          world: beginnning_state,
          current_slice_ptr: 0
        },
      ],
    }
  }

  pub fn to_image(self: Self, path: PathBuf) {
    let file = File::create(path);
    match file {
      Ok(mut fd) => {
        let _ = fd.write(b"P1\n");
        let _ = write!(fd, "{} {}\n", self.state[0].world.len(), self.state.len());
        for s in self.state {
          let mut w: String = "".into();
          for b in s.world {
            w.push_str( if b {"1"} else {"0"} )
          }
          let _ = write!(fd, "{}",w);
          let _ = write!(fd, "\n");
        }
      }
      Err(e) => eprintln!("Could no create file due to {:?}", e),
    }
  }
}

