One_d is small self-contained simulator for a subset of elementary cellular automata.

The world is some binary array of cells, i.e. each cell contains `true` or `false`.

In each iteration a new array is created as a function of the previous state array.

I.e. we split the world into chunks of a specified size and based on given patterns add the functions output to a new state.

For more information see (Elementary Cellular Automata)[https://en.wikipedia.org/wiki/Elementary_cellular_automaton#]


# Running the program
This program has no dependdencies except for standard rust. 
Game state is saved as a .p1 image, which can, for example, be viewed in (feh)[https://feh.finalrewind.org/]
```
cargo run && feh --force-aliasing game_rule_110.p1
```
## Options
