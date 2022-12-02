# Rxi's Microui Port to Rust

This a port of Rxi's MicroUI to Rust language. 
We tried to keep the usage pattern as close to the original as possible, but also as idiomatic to Rust as possible. By no mean this library should be considered complete.

We used C2Rust to create the initial code and iterated > 40 times to get it to where it is now. Few bugs are lingering (Lost to translation!), be advised!

## Demo
Clone and build the demo:
```
$ cd demo-no_std
$ cargo run --release
```

![random](https://github.com/NeoCogi/microui-rs/raw/master/res/microui.png)

After running `strip`, the executable should be around `70k` in size.

## Caveats
We used `strtod` and `sprintf` to keep the size down with `no_std`. Note that is possible to use `format_args!` to achieve similar behaviour at the cost of more bytes (~`110k`). We opt to remove this from the release for now.

