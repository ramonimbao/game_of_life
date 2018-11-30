#Game of Life

Yet another Rust implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

##Prerequisites

Make sure to follow the the [install instructions](https://github.com/Rust-SDL2/rust-sdl2) for `rust-sdl2` before building with `cargo build --release`. 

##Using

Run the program with `cargo run --release`. 

The program runs in a paused state initially. Clicking on the screen lets you choose which cells are alive or dead.

**SPACEBAR** unpauses and pauses the simulation.

**PERIOD (.)** steps through the simulation.

**R** randomizes all the cells.

**C** or **BACKSPACE** clears all the cells.

**UP/DOWN** arrow keys changes the speed of the simulation while running.

##License

This project is licensed under the MIT license - see the [LICENSE.md](LICENSE.md) for details.