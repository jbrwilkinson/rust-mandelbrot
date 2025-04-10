# Mandelbrot Example from 'Programming Rust' book

This is a fixed-up, modular version of the `Mandelbrot` example code in the 
['Programming Rust' book by O'Reilly](https://learning.oreilly.com/library/view/programming-rust/9781491927274/)
whose sources are [publicly-available here](https://github.com/ProgrammingRust/).

[![Example](./example_small.webp)](./example_large.png)

## Build, Test, Run

```bash
cargo build
cargo test
cargo run mandel.png 1920X1080 -1.20,0.35 -1,0.20
```

...outputs to `mandel.png` in the current folder
