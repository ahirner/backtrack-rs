use crate::problem::{Check, Scope};

#[derive(Debug)]
pub struct NQueens {
    n: usize,
}

impl NQueens {
    pub fn new(n: usize) -> Self {
        NQueens { n }
    }
}

impl Scope for NQueens {
    fn size(&self) -> usize {
        self.n
    }

    fn domain(&self) -> Vec<usize> {
        (0..self.n).collect()
    }
}

impl Check for NQueens {
    fn extends_sat(&self, solution: &[usize], x: usize) -> bool {
        let k = solution.len();
        for (j, x_j) in solution.iter().enumerate() {
            if x == *x_j {
                return false;
            }
            let diag = (x as isize - *x_j as isize).abs() as usize == k - j;
            if diag {
                return false;
            }
        }
        true
    }
}
