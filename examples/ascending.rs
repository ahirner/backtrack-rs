use back_rs::problems::Ascending;
use back_rs::solvers::IterSolveNaive;

fn main() {
    let asc = Ascending::new(4, 0..5);
    println!("backtrack.rs: {:?}", asc);

    let solver = IterSolveNaive::new(&asc);
    for solution in solver.into_iter() {
        println!("{:?}", solution);
    }
}
