# Rust nes emulator

This repo contains code for my toy Nes emulator. At this moment it supports all official CPU opcodes except interrupts (BRK & RTI instructions).

Development of this project is suspended (PPU was in progress) as its main purpose was to practice Rust in more advanced project (especially the macros) and this goal is accomplished.
At branch `ppu` you can find recent WIP code for Picture Processing Unit.

# Try it yourself

```
# Run example from binary code in Rust file (control by WASD)
cargo run --example snake

# or try the same game from iNES 1.0 format rom file
cargo run --example snake_rom
```
