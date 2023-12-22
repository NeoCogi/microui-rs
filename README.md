# Rxi's Microui Port to Rust
[![Crate](https://img.shields.io/crates/v/microui.svg)](https://crates.io/crates/microui)

This a port of Rxi's MicroUI to Rust language. 
We tried to keep the usage pattern as close to the original as possible, but also as idiomatic to Rust as possible. By no mean this library should be considered complete.

We used C2Rust to create the initial code and iterated > 60 times to get it to where it is now. Few bugs are lingering (Lost to translation!), be advised!

## Approach taken
This is a relatively small project. And to be honest, it did not start with c2rust. 

#### First Step
Rather, we forked microui and made few changes to the C code to make the generated more understandable. The fork can be found [here](https://github.com/eloraiby/microui/commits/jump_int). The C code modification mainly convert the pointer jumping to index jumping.

#### C2Rust
Otherwise, we have done the following:
1. Use stable rust:
   1. By default, c2rust generates code that builds with nightly. Our aim is to build using stable rust, that was our first milestone.
   2. C2Rust generates a fake main, so we removed the boiler-plate
2. Start porting C Enums (constants when translated to rust) to rust enums
3. Use containers (if the aim is to use `std`, then use the rust standard library collections/containers). For us, to be faithful to the original design of microui, our objective is to provide a zero allocation microui. We created our own non-alloc containers for that.
4. Move from C strings to rust strings
5. Finally, change all C types to rust compatible types.

## Demo
Clone and build the demo (SDL2 & glow) / Tested on linux:
```
$ cargo run --example demo-sdl2
```

![random](https://github.com/NeoCogi/microui-rs/raw/master/res/microui.png)

## Caveats
We opt to use our own implementations for parsing/serializing decimals (not general purpose floats). The decimal representation is stored as float. This is not a general purpose floating point parser/serializer and the algorithm is rather naive. Keep that in mind!
