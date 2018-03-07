use std::cmp::PartialOrd;
use std::hash::Hash;
use std::ops::Index;

pub trait Dataset where Self: Sized, <Self::Sample as Index<usize>>::Output: PartialOrd<u8> {
    type Sample: Index<usize>;
    type Label: Copy + Hash + Eq;

    fn feature_max() -> usize;
    fn threshold_max() -> u8;

    fn load(path: &str) -> Result<Self, String>;
    fn train_data<'a>(&'a self) -> (&[Self::Sample], &[Self::Label]);
    fn test_data<'a>(&'a self) -> (&[Self::Sample], &[Self::Label]);
}

pub mod mnist;
