use backtrack::problems::TotalSum;
use backtrack::solvers::IterSolveCached;

fn main() {
    let sums = TotalSum::new(4, &[4, 0, 1, 2], 4);
    eprintln!("total_sum.rs: {:?}", sums);

    let solver = IterSolveCached::new(&sums);
    for solution in solver.into_iter() {
        println!("{:?}", solution);
    }
}
