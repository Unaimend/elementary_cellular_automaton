mod game;
mod pattern;
mod world;
use game::*;
use pattern::*;
use std::path::PathBuf;
use crate::world::World;

// TODO Impl Center runner
//# One with 1,2,3 as states and modulo to get the next generation
struct CenterRunner<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> {}
impl<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> CenterRunner<WORLD_SIZE, PATTERN_SIZE> {
  fn run(&mut self, game: &mut Game<WORLD_SIZE, PATTERN_SIZE, Pattern3to1<PATTERN_SIZE>>) {
    let mut new_state: [bool; WORLD_SIZE] = [false; WORLD_SIZE];
    for r in &game.rules {
      let mut ctr: usize = 0;
      loop {
        let chunk = game.state.last_mut().unwrap().get_chunks_of_size_cyclig();
        if let Some(c) = chunk {
          if c == r.in_pattern {
            new_state[ctr] = r.out_pattern;
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


struct NonCenterRunner<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> {}

impl<const WORLD_SIZE: usize, const PATTERN_SIZE: usize> NonCenterRunner<WORLD_SIZE, PATTERN_SIZE> {
  fn run(&mut self, game: &mut Game<WORLD_SIZE, PATTERN_SIZE, Pattern3to3<PATTERN_SIZE>>) {
    let mut new_state: [bool; WORLD_SIZE] = [false; WORLD_SIZE];
    for r in &game.rules {
      let mut ctr: usize = 0;
      loop {
        let chunk = game.state.last_mut().unwrap().get_chunks_of_size();
        if let Some(c) = chunk {
          if *c == r.in_pattern {
            new_state[ctr..ctr + PATTERN_SIZE].copy_from_slice(&r.out_pattern);
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

pub fn generate_rules(rule_number: u8) -> Vec<Pattern3to1<3>> {
    let mut rules = Vec::with_capacity(8);

    // The 8 possible 3-bit neighborhoods (from 111 down to 000)
    let neighborhoods: [[bool; 3]; 8] = [
        [true, true, true],
        [true, true, false],
        [true, false, true],
        [true, false, false],
        [false, true, true],
        [false, true, false],
        [false, false, true],
        [false, false, false],
    ];

    // The output for each neighborhood corresponds to a bit in the rule_number.
    // Index 0 (111) corresponds to bit 7, Index 7 (000) corresponds to bit 0.
    for (i, in_pattern) in neighborhoods.into_iter().enumerate() {
        // Calculate which bit of the rule_number corresponds to this neighborhood.
        // Bit index = 7 - i
        let bit_index = 7 - i;
        
        // Check if the bit is set (i.e., the output is true)
        let out_pattern = (rule_number & (1 << bit_index)) != 0;

        rules.push(Pattern3to1 { in_pattern, out_pattern });
    }

    rules
}

fn main() {
    // Constants for the simulation
    const PATTERN_SIZE: usize = 3;
    const WORLD_SIZE: usize = 1000;
    const GENERATIONS: u32 = 256;

    let rules = generate_rules(110);


    // --- Initialize Game and Runner ---
    let mut game: Game<WORLD_SIZE, PATTERN_SIZE, Pattern3to1<PATTERN_SIZE>> = 
        Game::new(rules);
    game.state[0].world[WORLD_SIZE / 2] = true; 
    game.state[0].world[WORLD_SIZE / 2-1] = true; 
    game.state[0].world[WORLD_SIZE / 2+1] = true; 
    
    let mut runner = CenterRunner::<WORLD_SIZE, PATTERN_SIZE> {};

    // --- Run Simulation ---
    for i in 0..GENERATIONS {
        println!("Generating step {}", i + 1);
        runner.run(&mut game);
    }

    // --- Output to File ---
    let output_path: PathBuf = "game_rule_110.p1".into();
    game.to_image(output_path);
    println!("Simulation complete. Output written to {}", "game_rule_110.p1");
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
//

  //let r1 = Pattern3to3 {
  //  in_pattern: [false, false, true, false, false],
  //  out_pattern: [false, true, true, true, false],
  //};

  //let mut g: Game<WORLD_SIZE, PATTERN_SIZE, Pattern3to3<PATTERN_SIZE>> = Game::new(vec![r1]);
  //let mut r = NonCenterRunner {};
  //g.state[0].world[499] = true;
  //g.state[0].world[498] = true;
  //g.state[0].world[250] = true;
  //g.state[0].world[250] = true;
  //g.state[0].world[0] = true;
  //g.state[0].world[1] = true;
  //for i in 0..40 {
  //  println!("{:?}", i);
  //  r.run(&mut g)
  //}
  //g.to_image("game.p1".into());
  //
  fn rule110() {
    const PATTERN_SIZE: usize = 3;
    const WORLD_SIZE: usize = 100;
    const GENERATIONS: u32 = 256;

    // --- Define Rule 110 (Elementary CA, 3-cell neighborhood, 1-cell output) ---
    // Rule 110 (01101110 in binary)
    // 111 -> 0 (Rule 0)
    let r1 = Pattern3to1 { in_pattern: [true, true, true], out_pattern: false };
    // 110 -> 1 (Rule 1)
    let r2 = Pattern3to1 { in_pattern: [true, true, false], out_pattern: true };
    // 101 -> 1 (Rule 1)
    let r3 = Pattern3to1 { in_pattern: [true, false, true], out_pattern: true };
    // 100 -> 0 (Rule 0)
    let r4 = Pattern3to1 { in_pattern: [true, false, false], out_pattern: false };
    // 011 -> 1 (Rule 1)
    let r5 = Pattern3to1 { in_pattern: [false, true, true], out_pattern: true };
    // 010 -> 1 (Rule 1)
    let r6 = Pattern3to1 { in_pattern: [false, true, false], out_pattern: true };
    // 001 -> 1 (Rule 1)
    let r7 = Pattern3to1 { in_pattern: [false, false, true], out_pattern: true };
    // 000 -> 0 (Rule 0, handled by default)
    
    //let rules = vec![r1, r2, r3, r4, r5, r6, r7];
    let rules = vec![r1, r2, r3, r4, r5, r6, r7];


    // --- Initialize Game and Runner ---
    let mut game: Game<WORLD_SIZE, PATTERN_SIZE, Pattern3to1<PATTERN_SIZE>> = 
        Game::new(rules);
    game.state[0].world[WORLD_SIZE / 2] = true; 
    
    let mut runner = CenterRunner::<WORLD_SIZE, PATTERN_SIZE> {};

    // --- Run Simulation ---
    for i in 0..GENERATIONS {
        println!("Generating step {}", i + 1);
        runner.run(&mut game);
    }

    // --- Output to File ---
    let output_path: PathBuf = "game_rule_110.p1".into();
    game.to_image(output_path);
    println!("Simulation complete. Output written to {}", "game_rule_110.p1");
  }
