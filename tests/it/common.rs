use std::fmt::Debug;

pub(crate) fn assert_unsat_unique<T>(unsats: T)
where
    T: IntoIterator,
    T::Item: PartialEq + Debug,
{
    let mut all = Vec::new();
    for u in unsats.into_iter() {
        assert!(!all.contains(&u), "{:?} appeared a second time", &u);
        all.push(u);
    }
}
