use std::error::Error;

use backtrack_rs::problems::NQueens;
use backtrack_rs::solvers::IterSolveNaive;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let n = match args.len() {
        2 => args[1].parse()?,
        _ => return Err(format!("Use: {} <n>", &args[0]).into()),
    };

    let n_queens = NQueens::new(n);
    eprintln!("n_queens.rs: {:?}", n_queens);

    let solver = IterSolveNaive::new(&n_queens);
    for solution in solver.into_iter() {
        println!("{:?}", solution);
    }
    Ok(())
}
