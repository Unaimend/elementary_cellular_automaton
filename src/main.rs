use std::{fs::File, io::Write, path::PathBuf};
const WORLD_SIZE: usize = 9;
#[derive(Clone, Copy, Debug)]
struct World {
  world: [bool; WORLD_SIZE],
}

impl Default for World {
  fn default() -> Self {
    World { world: [false; WORLD_SIZE] }
  }

}


struct Pattern {
  id: String,
  in_pattern: [bool; 3],
  out_pattern: [bool; 3],
}

struct Runner {
  current_iteration: usize,
}

impl Runner {
  fn run(&mut self, game: &mut Game) {
    for r in &game.rules {
      let chunks = game.state[self.current_iteration].world.chunks_exact_mut(3);
      for chunk in chunks {
        // chunks of 3 elements
        println!("in_pattern {:?}, chunk {:?}", r.in_pattern, &chunk[..]);
        if r.in_pattern == chunk[..] {
          chunk.copy_from_slice(&r.out_pattern);
          println!("Rule {} hit", r.id);
        }
      }
      println!("----------------");
    }
    // now clone the **modified** state
    let new_state = game.state[self.current_iteration].clone();
    println!("New state {:?}", new_state);
    game.state.push(new_state);
    self.current_iteration += 1;}
}

struct Game {
  rules: Vec<Pattern>,
  state: Vec<World>,
}

impl Game {
  fn new(rules: Vec<Pattern>) -> Self {
    let beginnning_state = [false; WORLD_SIZE];
    Game {
      rules,
      state: vec![
        World {
          world: beginnning_state,
        },
        World {
          world: beginnning_state,
        },
      ],
    }
  }

  fn to_image(self: Self, path: PathBuf) {
    let file = File::create(path);
    match file {
      Ok(mut fd) => {
        let _ = fd.write(b"P1\n");
        let _ = write!(fd, "{} {}\n", self.state[0].world.len(), self.state.len());
        for s in self.state {
          for b in s.world {
            let _ = write!(fd, "{} ", if b { 1 } else { 0 });
          }
          let _ = write!(fd, "\n");
        }
      }
      Err(e) => eprintln!("Could no create file due to {:?}", e),
    }
  }
}

fn main() {
  let r1 = Pattern {
    id: "r1".into(),
    in_pattern: [false, true, false],
    out_pattern: [false, false, true],
  };

  let r2 = Pattern {
    id: "r2".into(),
    in_pattern: [true, false , false],
    out_pattern: [false, true, false],
  };


  let r3 = Pattern {
    id: "r3".into(),
    in_pattern: [true, false, false],
    out_pattern: [false, true, false],
  };

  let r3 = Pattern {
    id: "r3".into(),
    in_pattern: [false, false, true],
    out_pattern: [true, false , false],
  };
  let mut g = Game::new(vec![r1, r2, r3]);
  let mut r = Runner {
    current_iteration: 1,
  };
  g.state[0].world[1] = true;
  g.state[1].world[1] = true;
  for i in 0..2{
    r.run(&mut g)
  }
  g.to_image("game.p1".into());
}
