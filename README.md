# backtrack-rs 🦀
[![Documentation](https://docs.rs/backtrack/badge.svg)](https://docs.rs/backtrack)
[![crates.io](https://img.shields.io/crates/v/backtrack.svg)](https://crates.io/crates/backtrack)
[![CI](https://github.com/ahirner/backtrack-rs/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/ahirner/backtrack-rs/actions/workflows/rust-ci.yml)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)
<!-- cargo-sync-readme start -->

`backtrack` lets you solve [backtracking](https://en.wikipedia.org/wiki/Backtracking) problems
simply and generically.

Problems are defined by their *scope* and *checks* against possible solutions.

A [Scope](https://docs.rs/backtrack/latest/backtrack/problem/trait.Scope.html) determines length and allowed values of a solution.
The domain defaults to `usize`, but any `T` works if it lives as long as its `Scope`, including references.

The [Check](https://docs.rs/backtrack/latest/backtrack/problem/trait.Check.html) or [CheckInc](https://docs.rs/backtrack/latest/backtrack/problem/trait.CheckInc.html) trait determines whether
a particular combination of values is satisfying.


## Usage

It is required that solutions shorter than the entire scope, i.e. partial
solutions must satisfy if the completed solutions should as well.

[Solvers](https://docs.rs/backtrack/latest/backtrack/solvers/) borrow a problem in search for
[candidate solutions](https://docs.rs/backtrack/latest/backtrack/solve/enum.CandidateSolution.html).

### Checks
We define the problem of counting down with a limited set of numbers and solve iteratively.
```rust
use backtrack::problem::{Check, Scope};
use backtrack::solvers::IterSolveNaive;
// helper trait to filter solutions of interest
use backtrack::solve::IterSolveExt;

/// Obtain permutations of some 3 descending numbers
struct CountDown {}

impl Scope<'_> for CountDown {
    fn size(&self) -> usize { 3 }
    fn value(&self, index: usize) -> usize { index }
    fn len(&self) -> usize { 4 }
}

impl Check for CountDown{
    fn extends_sat(&self, solution: &[usize], x: &usize) -> bool {
        solution.last().map_or(true, |last| *last > *x)
    }
}

let solver = IterSolveNaive::new(&CountDown{});
let mut sats = solver.sat_iter();

assert_eq!(sats.next(), Some(vec![2, 1, 0]));
assert_eq!(sats.next(), Some(vec![3, 1, 0]));
assert_eq!(sats.next(), Some(vec![3, 2, 0]));
assert_eq!(sats.next(), Some(vec![3, 2, 1]));
assert_eq!(sats.next(), None);
```
### Incremental Checks
If your checks can be formulated against a reduced solution,
implement [CheckInc](https://docs.rs/backtrack/latest/backtrack/problem/trait.CheckInc.html) instead.

The same result as above can be obtained by first "computing"
the last item at each step. Such an approach makes more sense if
work on more than one prior value needs to be peformed
for any given sat check.

```rust
use backtrack::problem::{CheckInc, Scope};
// ...
impl CheckInc for CountDown{
    type Accumulator = usize;

    fn fold_acc(&self, accu: Option<&Self::Accumulator>, x: &usize) -> Self::Accumulator {
        // only last value is of interest for checking
        *x
    }

    fn accu_sat(&self, accu: Option<&Self::Accumulator>, x: &usize, index: usize) -> bool {
       accu.map_or(true, |last| last > x)
    }
}
// since `CheckInc` impls `Check`, the same solver as before can be used
// todo: specialize solver to actually realize performance advantage
// ...
```

<!-- cargo-sync-readme end -->
## Examples
Checkout the `examples` folder for example problems.

```bash
# 4-queens solution
cargo run --example n_queens 4 | grep Sat
## n_queens.rs: NQueens { n: 4 }
## Sat([1, 3, 0, 2])
## Sat([2, 0, 3, 1])
```

```bash
# sequence of numbers which sum up to a minimum value but not more
cargo run --example total_sum | grep Sat
```
## Benchmarks
`backtrack-rs` uses [criterion](https://crates.io/crates/criterion) for benches.
```bash
cargo bench
```

## Todos
- [ ] `CheckInc` solver
- [ ] parallelize search
