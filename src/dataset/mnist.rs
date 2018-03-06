use std::ops::Index;

use dataset::Dataset;

// An Image is a 28x28 handwritten digit.
// A value of 255 means "black", while 0 means "white".
#[derive(Copy, Clone)]
struct Image([u8; 784]);

impl Index<usize> for Image {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &self.0[index]
    }
}

/// A loaded copy of the MNIST dataset.
struct MNIST {
    train_data: [Image; 60000],
    test_data: [Image; 10000],

    train_labels: [u8; 60000],
    test_labels: [u8; 10000]
}

impl Dataset for MNIST {
    type Sample = Image;
    type Label = u8;

    /// Load the dataset from a directory.
    /// The directory should contain the four files:
    /// - train-images-idx3-ubyte
    /// - train-labels-idx1-ubyte
    /// - t10k-images-idx3-ubyte
    /// - t10k-labels-idx1-ubyte
    fn load(path: &str) -> Result<Self, String> {
        // TODO: load the data from files here.
        Ok(MNIST{
            train_data: [Image{0: [0; 784]}; 60000],
            test_data: [Image{0: [0; 784]}; 10000],
            train_labels: [0; 60000],
            test_labels: [0; 10000]
        })
    }

    fn train_data<'a>(&'a self) -> (&[Self::Sample], &[Self::Label]) {
        (&self.train_data, &self.train_labels)
    }

    fn test_data<'a>(&'a self) -> (&[Self::Sample], &[Self::Label]) {
        (&self.test_data, &self.test_labels)
    }
}
