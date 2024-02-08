## Description
Rust version - 1.75.0

Rust project consists of two packages:
- graph_lib - lib, which provides graph realizations;
- usage - binary, which deserializes graph from [Trivial Graph Format](https://en.wikipedia.org/wiki/Trivial_Graph_Format) and print it in [Depth-first search](https://en.wikipedia.org/wiki/Depth-first_search) way using lib.

From the root of the project to run binary:
`cargo run`

To build certain package in release mode:
`cargo build -p graph_lib --release`