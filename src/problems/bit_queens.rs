use crate::problem::{CheckInc, Scope};
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct BitQueens {
    n: usize,
}

impl BitQueens {
    /// Returns largest solvable size
    pub fn max_n() -> usize {
        std::mem::size_of::<usize>() * 8
    }
    pub fn new(n: usize) -> Self {
        let bit_width = Self::max_n();
        assert!(n <= bit_width, "Only problem sizes up to {} supported (usize width)", bit_width);
        BitQueens { n }
    }
}

#[derive(Copy, Clone)]
struct UBitVec(usize);

impl Debug for UBitVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl Scope<'_> for BitQueens {
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

#[derive(Debug, Clone)]
pub struct QueenField {
    straight: UBitVec,
    diagonal_left: UBitVec,
    diagonal_right: UBitVec,
    contended: bool,
}

impl CheckInc for BitQueens {
    type Accumulator = QueenField;

    fn fold_acc(
        &self,
        accu: Option<Self::Accumulator>,
        x: &usize,
        _position: usize,
    ) -> Self::Accumulator {
        let one_hot = 1 << x;

        if let Some(mut accu) = accu {
            let diagonal_left = accu.diagonal_left.0 << 1;
            let diagonal_right = accu.diagonal_right.0 >> 1;
            let all = diagonal_left | diagonal_right | accu.straight.0;

            if (all & one_hot) > 0 {
                accu.contended = true;
                return accu;
            }

            accu.straight.0 |= one_hot;
            accu.diagonal_left.0 = diagonal_left | one_hot;
            accu.diagonal_right.0 = diagonal_right | one_hot;

            accu
        } else {
            QueenField {
                straight: UBitVec(one_hot),
                diagonal_left: UBitVec(one_hot),
                diagonal_right: UBitVec(one_hot),
                contended: false,
            }
        }
    }

    fn accu_sat(&self, accu: &Self::Accumulator, _x: &usize, _position: usize) -> bool {
        !accu.contended
    }
}
