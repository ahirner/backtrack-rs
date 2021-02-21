pub trait Problem {
    /// Return size of the problem
    fn get_n(&self) -> usize;
    /// Return all possible solution values in order
    fn get_domain(&self) -> Vec<usize>;

    /// Returns true if a new value extends a valid partial solution
    ///
    /// # Arguments
    /// * `solution`: candidate partial solution
    /// * `x_l`: new solution value
    fn inc_sat(&self, solution: &[usize], x_l: usize) -> bool;

    // todo: provide some caching facility for `inc_sat`
}
