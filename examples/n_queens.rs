use std::error::Error;

use backtrack::problems::{BitQueens, NQueens};
use backtrack::solvers::{IterSolveCached, IterSolveNaive};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let usage = Err(format!(
        "Use: {} <n> <bit|int> (defaults to `bit` up to usize width)",
        &args[0]
    )
    .into());

    let (n, bit) = match args.len() {
        2 => {
            let n = args[1].parse()?;
            let bit = n <= BitQueens::max_n();
            (n, bit)
        }
        3 => {
            let n = args[1].parse()?;
            let bit = match &args[2][..] {
                "bin" => true,
                "int" => false,
                _ => return usage,
            };
            (n, bit)
        }
        _ => return usage,
    };

    if !bit {
        let n_queens = NQueens::new(n);
        eprintln!("n_queens.rs: {:?}", n_queens);

        let solver = IterSolveNaive::new(&n_queens);
        for solution in solver {
            println!("{:?}", solution);
        }
    } else {
        let bit_queens = BitQueens::new(n);
        eprintln!("bit_queens.rs: {:?}", bit_queens);

        let solver = IterSolveCached::new(&bit_queens);
        for solution in solver {
            println!("{:?}", solution);
        }
    }
    Ok(())
}
