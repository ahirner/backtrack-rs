use backtrack::problems::CountUp;
use backtrack::solvers::IterSolveNaive;

fn main() {
    let asc = CountUp::new(4, 0..5);
    eprintln!("count_up.rs: {:?}", asc);

    let solver = IterSolveNaive::new(&asc);
    for solution in solver.into_iter() {
        println!("{:?}", solution);
    }
}
