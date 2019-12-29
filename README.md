# fuzz-robustness-orientation

Tool that verifies correctness between [Spade’s `exactpred.rs`](https://github.com/Stoeoef/spade/blob/master/src/exactpred.rs) and [Jonathan Richard Shewchuk’s `predicates.c`](https://www.cs.cmu.edu/~quake/robust.html). The former is a Rust rewrite of the latter. More background in [this conversation](https://github.com/georust/geo/issues/380).

## Run

`cargo run --release`
