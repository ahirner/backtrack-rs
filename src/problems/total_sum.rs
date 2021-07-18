use crate::problem::{CheckInc, Scope};

/// Accept all values where the final sum was reached with one non-zero solution.
///
/// If e.g. `sum_at_least` is 3 and 4 arbitrary integers are required, [0,2,2,0],
/// [1,1,0,1] and [2,0,3,0] are valid while [1,1,1,1] is not.
#[derive(Debug)]
pub struct TotalSum {
    n: usize,
    domain: Vec<usize>,
    sum_at_least: usize,
}

impl TotalSum {
    pub fn new(n: usize, domain: &[usize], sum_at_least: usize) -> Self {
        TotalSum { n, domain: domain.to_vec(), sum_at_least }
    }
}

impl Scope<'_> for TotalSum {
    fn size(&self) -> usize {
        self.n
    }

    fn value(&self, index: usize) -> usize {
        self.domain[index]
    }

    fn len(&self) -> usize {
        self.domain.len()
    }
}

#[derive(Debug, Clone)]
pub struct SumReached {
    sum: usize,
    last_satisfied: bool,
    satisfied: bool,
}

impl CheckInc for TotalSum {
    type Accumulator = SumReached;
    fn fold_acc(
        &self,
        accu: Option<Self::Accumulator>,
        x: &usize,
        _position: usize,
    ) -> Self::Accumulator {
        let mut accu = accu.unwrap_or(SumReached {
            sum: 0,
            last_satisfied: self.sum_at_least == 0,
            satisfied: false,
        });
        accu.sum += *x;
        accu.last_satisfied = accu.satisfied;
        accu.satisfied = accu.sum >= self.sum_at_least;
        accu
    }

    fn accu_sat(&self, accu: &Self::Accumulator, x: &usize, position: usize) -> bool {
        let last_satisfied = accu.last_satisfied;

        // reject incomplete solutions early iff non-zero addition and last was already satisfied
        if position < self.size() - 1 {
            !(*x > 0 && accu.satisfied && last_satisfied)
        } else if *x > 0 {
            (!last_satisfied) && accu.satisfied
        } else {
            accu.satisfied
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problems::utils::sat_safe;

    #[test]
    fn sums_sat3() {
        let sums = TotalSum::new(3, &[4, 2, 0, 1], 4);
        assert!(sat_safe(&sums, vec![4]));
        assert!(sat_safe(&sums, vec![0, 0, 4]));
        assert!(sat_safe(&sums, vec![1, 1, 4]));
        assert!(sat_safe(&sums, vec![2, 1, 1]));
        assert!(sat_safe(&sums, vec![4, 0, 0]));
    }
    #[test]
    fn sums_sat4() {
        let sums = TotalSum::new(4, &[4, 2, 0, 1], 3);
        assert!(sat_safe(&sums, vec![4]));
        assert!(sat_safe(&sums, vec![0, 2, 2, 0]));
        assert!(sat_safe(&sums, vec![1, 1, 0, 1]));
        assert!(sat_safe(&sums, vec![2, 0, 2, 0]));
    }

    #[test]
    fn sums_unsat_too_few() {
        let sums = TotalSum::new(3, &[4, 0, 1], 4);
        assert!(!sat_safe(&sums, vec![0, 0, 0]));
        assert!(!sat_safe(&sums, vec![0, 1, 1]));
        assert!(!sat_safe(&sums, vec![1, 1, 1]));
    }
    #[test]
    fn sums_unsat_too_many() {
        let sums = TotalSum::new(3, &[4, 0, 1], 4);
        assert!(!sat_safe(&sums, vec![4, 1, 1]));
        assert!(!sat_safe(&sums, vec![4, 0, 1]));
        assert!(!sat_safe(&sums, vec![1, 4, 1]));
    }
}
