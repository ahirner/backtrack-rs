use crate::problem::Problem;
use std::ops::Range;

/// Accept `n` numbers where each number must be
/// greater than former and from `domain`
#[derive(Debug)]
pub struct Ascending {
    n: usize,
    domain: Range<usize>,
}

impl Ascending {
    pub fn new(n: usize, domain: Range<usize>) -> Self {
        Ascending { n, domain }
    }
}

impl Problem for Ascending {
    fn get_n(&self) -> usize {
        self.n
    }

    fn get_domain(&self) -> Vec<usize> {
        self.domain.clone().collect()
    }

    fn inc_sat(&self, solution: &[usize], x_l: usize) -> bool {
        if let Some(last) = solution.last() {
            x_l > *last
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problems::utils::sat_safe;

    #[test]
    fn ascending_sat() {
        let asc = Ascending::new(4, 0..4);
        assert!(sat_safe(&asc, 0..0));
        assert!(sat_safe(&asc, 0..4));
        assert!(sat_safe(&asc, 2..4));
    }
    #[test]
    fn ascending_unsat() {
        let asc = Ascending::new(4, 0..4);
        assert!(!sat_safe(&asc, (0..2).rev()));
        assert!(!sat_safe(&asc, (0..4).rev()));
    }
}
