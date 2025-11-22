use std::{fs::File, io::Write, path::PathBuf};

mod world;
mod pattern;
mod game;
use game::*;
use pattern::*;


struct Runner<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> {
  current_iteration: usize,
}

impl<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> Runner<WORLD_SIZE, PATTERN_SIZE> {
  fn run(&mut self, game: &mut Game<WORLD_SIZE, PATTERN_SIZE>) {
    for r in &game.rules {
      loop {
        let mut chunk: Option<&mut [bool; PATTERN_SIZE]> = game.state[self.current_iteration].get_chunks_of_size();
        if let Some(c) = &mut chunk {
          if **c == r.in_pattern {
            **c = r.out_pattern;
          }
          dbg!(**c);
          dbg!("-----------");
        } else {
          break;
        }
      }
    }
    println!("----------------");
    // now clone the **modified** state
    let new_state = game.state[self.current_iteration].clone();
    println!("New state {:?}", new_state);
    game.state.push(new_state);
    self.current_iteration += 1;
  }
}

fn main() {
const PATTERN_SIZE: usize = 3;
const WORLD_SIZE: usize = 10;
  let r1 = Pattern {
    id: "r1".into(),
    in_pattern: [false, true, false],
    out_pattern: [false, false, true],
  };
  
  let mut g: Game<WORLD_SIZE, PATTERN_SIZE> = Game::new(vec![r1]);
  let mut r = Runner {
    current_iteration: 1,
  };
  g.state[0].world[1] = true;
  g.state[1].world[1] = true;
  for i in 0..3{
    r.run(&mut g)
  }
  g.to_image("game.p1".into());
}
