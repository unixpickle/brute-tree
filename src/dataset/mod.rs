use std::cmp::PartialOrd;
use std::io;
use std::ops::Index;

pub trait Dataset where Self: Sized, <Self::Sample as Index<usize>>::Output: PartialOrd<u8> {
    type Sample: Index<usize>;

    fn feature_max() -> usize;
    fn threshold_max() -> u8;

    fn load(path: &str) -> Result<Self, io::Error>;
    fn train_data<'a>(&'a self) -> (&[Self::Sample], &[usize]);
    fn test_data<'a>(&'a self) -> (&[Self::Sample], &[usize]);
}

pub mod mnist;
