use std::{fs::File, io::Write, path::PathBuf};

mod world;
mod pattern;
mod game;
use game::*;
use pattern::*;

use crate::world::World;


struct Runner<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> {
  current_iteration: usize,
}

impl<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> Runner<WORLD_SIZE, PATTERN_SIZE> {
  fn run(&mut self, game: &mut Game<WORLD_SIZE, PATTERN_SIZE>) {
    println!("Old state {:?}", game.state);
    let mut new_state: [bool; WORLD_SIZE] = [false; WORLD_SIZE];
    for r in &game.rules {
      loop {
        let ctr: usize = 0;
        let mut chunk = game.state.last_mut().unwrap().get_chunks_of_size();
        println!("Chunk {:?}", chunk);
        if let Some(c) = &mut chunk {
          if **c == r.in_pattern {
            new_state[ctr..ctr+PATTERN_SIZE].copy_from_slice(&r.out_pattern);
          }
        } else {
          break;
        }
      }
    }
    println!("----------------");
    // now clone the **modified** state
    println!("New state {:?}", new_state);
    game.state.push(World::new(new_state));
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
    current_iteration: 0,
  };
  g.state[0].world[1] = true;
  for i in 0..5{
    r.run(&mut g)
  }
  g.to_image("game.p1".into());
}
