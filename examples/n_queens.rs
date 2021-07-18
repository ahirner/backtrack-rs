use std::error::Error;

use backtrack::problems::{BinQueens, NQueens};
use backtrack::solvers::{IterSolveCached, IterSolveNaive};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let usage = Err(format!(
        "Use: {} <n> <bin|int> (defaults to `bin` up to usize width)",
        &args[0]
    )
    .into());

    let (n, bin) = match args.len() {
        2 => {
            let n = args[1].parse()?;
            let bin = n <= BinQueens::max_n();
            (n, bin)
        }
        3 => {
            let n = args[1].parse()?;
            let bin = match &args[2][..] {
                "bin" => true,
                "int" => false,
                _ => return usage,
            };
            (n, bin)
        }
        _ => return usage,
    };

    if !bin {
        let n_queens = NQueens::new(n);
        eprintln!("n_queens.rs: {:?}", n_queens);

        let solver = IterSolveNaive::new(&n_queens);
        for solution in solver.into_iter() {
            println!("{:?}", solution);
        }
    } else {
        let bin_queens = BinQueens::new(n);
        eprintln!("bin_queens.rs: {:?}", bin_queens);

        let solver = IterSolveCached::new(&bin_queens);
        for solution in solver.into_iter() {
            println!("{:?}", solution);
        }
    }
    Ok(())
}
