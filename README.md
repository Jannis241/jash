# jash

A small terminal emulator with a GUI, written in Rust. I built it on the side, mainly to try out [egui](https://github.com/emilk/egui).

You type a command, it runs through `sh -c` (or `cmd /C` on Windows), and the output is shown in the window. There is no real PTY, so interactive programs like `vim` or `htop` don't work (yet).

## Run

```sh
cargo run --release
```

## Config

At the moment the config is directly in the code, in `main.rs` (`window::Config`). There you can change the font size, colors, prompt, transparency and the cursor style (block, bar or underline).

## What I learned

- How to build a simple GUI with egui
- How to run shell commands from Rust and read their output
- This project could be really interesting in the future and i easily extendable, maybe i will get back to it.

