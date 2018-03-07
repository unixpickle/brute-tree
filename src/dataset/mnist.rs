use std::ops::Index;

use dataset::Dataset;

// An Image is a 28x28 handwritten digit.
// A value of 255 means "black", while 0 means "white".
#[derive(Copy, Clone)]
pub struct Image([u8; 784]);

impl Index<usize> for Image {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &self.0[index]
    }
}

/// A loaded copy of the MNIST dataset.
pub struct MNIST {
    train_data: Vec<Image>,
    test_data: Vec<Image>,

    train_labels: Vec<u8>,
    test_labels: Vec<u8>
}

impl Dataset for MNIST {
    type Sample = Image;
    type Label = u8;

    fn feature_max() -> usize {
        28 * 28 - 1
    }

    fn threshold_max() -> u8 {
        254
    }

    /// Load the dataset from a directory.
    /// The directory should contain the four files:
    /// - train-images-idx3-ubyte
    /// - train-labels-idx1-ubyte
    /// - t10k-images-idx3-ubyte
    /// - t10k-labels-idx1-ubyte
    fn load(path: &str) -> Result<Self, String> {
        // TODO: load the data from files here.
        Ok(MNIST{
            train_data: Vec::new(),
            test_data: Vec::new(),
            train_labels: Vec::new(),
            test_labels: Vec::new()
        })
    }

    fn train_data<'a>(&'a self) -> (&[Self::Sample], &[Self::Label]) {
        (self.train_data.as_slice(), self.train_labels.as_slice())
    }

    fn test_data<'a>(&'a self) -> (&[Self::Sample], &[Self::Label]) {
        (self.test_data.as_slice(), self.test_labels.as_slice())
    }
}
