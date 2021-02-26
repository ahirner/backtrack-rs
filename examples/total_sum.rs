use back_rs::problems::TotalSum;
use back_rs::solvers::IterSolveNaive;

fn main() {
    let sums = TotalSum::new(4, &[4, 0, 1, 2], 4);
    println!("total_sum.rs: {:?}", sums);

    let solver = IterSolveNaive::new(&sums);
    for solution in solver.into_iter() {
        println!("{:?}", solution);
    }
}
