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

impl Scope<'_> for NQueens {
    fn size(&self) -> usize {
        self.n
    }

    fn value(&self, index: usize) -> usize {
        index
    }

    fn len(&self) -> usize {
        self.size()
    }
}

impl Check for NQueens {
    fn extends_sat(&self, solution: &[usize], x: &usize) -> bool {
        let k = solution.len();
        for (j, x_j) in solution.iter().enumerate() {
            if x == x_j {
                return false;
            }
            let diag = (*x as isize - *x_j as isize).unsigned_abs() == k - j;
            if diag {
                return false;
            }
        }
        true
    }
}
