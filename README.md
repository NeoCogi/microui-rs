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
$ cd demo-sdl2
$ cargo run
```

![random](https://github.com/NeoCogi/microui-rs/raw/master/res/microui.png)

If you want to build the smallest executable:

```
$ cd demo-sdl2
$ cargo +nightly run --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu
```

This will give you a `127K` executable (with std)

## Caveats
We used `strtod` and `sprintf`. The reason for that:
1. `format_args` and derivatives don't support custom run time formatting
2. Keep the size down with `no_std`. Note that is possible to use `format_args!` to achieve similar behaviour at the cost of more bytes (~`110k`). We opt to remove this from the release for now.

