One_d is small self-contained simulator for elementary cellular automata.

The world is some binary array of cells, i.e. each cell contains `true` or `false`.

In each iteration a new array is created as a function of the previous state array.


# Running the program
This program has no dependdencies except for standard rust. 
Game state is saved as a .p1 image, which can, for example, be viewed in (feh)[https://feh.finalrewind.org/]
```
cargo run && feh --force-aliasing game_rule_110.p1
```
## Options
