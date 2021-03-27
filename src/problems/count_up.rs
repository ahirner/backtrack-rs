use crate::problem::{Check, Scope};
use std::ops::Range;

/// Accept `n` numbers where each number must be
/// greater than former and from `domain`
#[derive(Debug)]
pub struct CountUp {
    n: usize,
    domain: Range<usize>,
}

impl CountUp {
    pub fn new(n: usize, domain: Range<usize>) -> Self {
        CountUp { n, domain }
    }
}

impl Scope for CountUp {
    fn size(&self) -> usize {
        self.n
    }

    fn domain(&self) -> Vec<usize> {
        self.domain.clone().collect()
    }
}
impl Check for CountUp {
    fn extends_sat(&self, solution: &[usize], x: &usize) -> bool {
        if let Some(last) = solution.last() {
            *x > *last
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
    fn count_up_sat() {
        let asc = CountUp::new(4, 0..4);
        assert!(sat_safe(&asc, 0..0));
        assert!(sat_safe(&asc, 0..4));
        assert!(sat_safe(&asc, 2..4));
    }
    #[test]
    fn count_up_unsat() {
        let asc = CountUp::new(4, 0..4);
        assert!(!sat_safe(&asc, (0..2).rev()));
        assert!(!sat_safe(&asc, (0..4).rev()));
    }
}
