use crate::problem::{CheckInc, Scope};
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct BinQueens {
    n: usize,
}

impl BinQueens {
    /// Returns largest solvable size
    pub fn max_n() -> usize {
        std::mem::size_of::<usize>() * 8
    }
    pub fn new(n: usize) -> Self {
        let bin_width = Self::max_n();
        assert!(n < bin_width, "Only problem sizes up to {} supported (usize width)", bin_width);
        BinQueens { n }
    }
}

#[derive(Copy, Clone)]
struct UBinVec(usize);

impl Debug for UBinVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl Scope<'_> for BinQueens {
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
    straight: UBinVec,
    diagonal_left: UBinVec,
    diagonal_right: UBinVec,
    contended: bool,
}

impl CheckInc for BinQueens {
    type Accumulator = QueenField;

    fn fold_acc(
        &self,
        accu: Option<Self::Accumulator>,
        x: &usize,
        _position: usize,
    ) -> Self::Accumulator {
        let one_hot = 1 << x;

        if let Some(mut accu) = accu {
            if (accu.straight.0 & one_hot) > 0 {
                accu.contended = true;
                return accu;
            };
            let diagonal_left = accu.diagonal_left.0 << 1;
            let diagonal_right = accu.diagonal_right.0 >> 1;
            if ((diagonal_left | diagonal_right) & one_hot) > 0 {
                accu.contended = true;
                return accu;
            }

            accu.straight.0 |= one_hot;
            accu.diagonal_left.0 = (accu.diagonal_left.0 << 1) | one_hot;
            accu.diagonal_right.0 = (accu.diagonal_right.0 >> 1) | one_hot;

            accu
        } else {
            QueenField {
                straight: UBinVec(one_hot),
                diagonal_left: UBinVec(one_hot),
                diagonal_right: UBinVec(one_hot),
                contended: false,
            }
        }
    }

    fn accu_sat(&self, accu: &Self::Accumulator, _x: &usize, _position: usize) -> bool {
        !accu.contended
    }
}
