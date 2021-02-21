use back_rs::impls::Ascending;
use back_rs::solve::IterSolveNaive;

fn main() {
    let asc = Ascending::new(4, 0..5);
    println!("backtrack.rs: {:?}", asc);

    let solver = IterSolveNaive::new(&asc);
    for solution in solver.into_iter() {
        println!("{:?}", solution);
    }
}
