use std::{fs::File, io::Write, path::PathBuf};

mod world;
mod pattern;
mod game;
use game::*;
use pattern::*;

use crate::world::World;

//# CENTER version (the coreect one)
//# One with 1,2,3 as states and modulo to get the next generation

struct NonCenterRunner<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> {
  current_iteration: usize,
}

impl<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> NonCenterRunner<WORLD_SIZE, PATTERN_SIZE> {
  fn run(&mut self, game: &mut Game<WORLD_SIZE, PATTERN_SIZE>) {
    let mut new_state: [bool; WORLD_SIZE] = [false; WORLD_SIZE];
    for r in &game.rules {
      let mut ctr: usize = 0;
      loop {
        let chunk = game.state.last_mut().unwrap().get_chunks_of_size();
        if let Some(c) = chunk {
          if *c == r.in_pattern {
            new_state[ctr..ctr+PATTERN_SIZE].copy_from_slice(&r.out_pattern);
          }
        } else {
          break;
        }
      ctr += 1;
      }
    }
    game.state.push(World::new(new_state));
  }
}

fn main() {
  const PATTERN_SIZE: usize = 5;
  const WORLD_SIZE: usize = 500;
  let r1 = Pattern {
    id: "r1".into(),
    in_pattern: [false, false, true, false, false],
    out_pattern: [false, true, true, true, false],
  };

  let mut g: Game<WORLD_SIZE, PATTERN_SIZE> = Game::new(vec![r1]);
  let mut r = NonCenterRunner {
    current_iteration: 0,
  };
  g.state[0].world[499] = true;
  g.state[0].world[498] = true;
  g.state[0].world[250] = true;
  g.state[0].world[250] = true;
  g.state[0].world[0] = true;
  g.state[0].world[1] = true;
  for i in 0..40{
    println!("{:?}", i);
    r.run(&mut g)
  }
  g.to_image("game.p1".into());
}

//
//let r1 = Pattern {
//  id: "r1".into(),
//  in_pattern: [false, true, false],
//  out_pattern: [false, false, true],
//};
//let r2 = Pattern {
//  id: "r1".into(),
//  in_pattern: [false, false, true],
//  out_pattern: [false, true, false],
//};
//
//
// 
//
// THICKPINKSKI
//  let r1 = Pattern {
//    id: "r1".into(),
//    in_pattern: [false, true, false],
//    out_pattern: [true, true, true],
//  };
//  let r2 = Pattern {
//    id: "r1".into(),
//    in_pattern: [true, true, false],
//    out_pattern: [true, true, true],
//  };
//  let r3 = Pattern {
//    id: "r1".into(),
//    in_pattern: [false, true, true],
//    out_pattern: [true, true, true],
//  };
//
