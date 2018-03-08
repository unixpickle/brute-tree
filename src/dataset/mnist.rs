use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::Index;
use std::path::Path;

use dataset::Dataset;

const IMAGE_SIZE: usize = 28 * 28;

// An Image is a 28x28 handwritten digit.
// A value of 255 means "black", while 0 means "white".
#[derive(Copy, Clone)]
pub struct Image([u8; IMAGE_SIZE]);

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

    train_labels: Vec<usize>,
    test_labels: Vec<usize>
}

impl Dataset for MNIST {
    type Sample = Image;

    fn feature_max() -> usize {
        IMAGE_SIZE - 1
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
    fn load(path: &str) -> Result<Self, io::Error> {
        let child_path = |x| -> Result<String, io::Error>{
            let joined = Path::new(path).join(Path::new(x));
            if let Some(x) = joined.to_str() {
                Ok(String::from(x))
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "invalid path"))
            }
        };
        Ok(MNIST{
            train_data: read_image_file(&child_path("train-images-idx3-ubyte")?)?,
            test_data: read_image_file(&child_path("t10k-images-idx3-ubyte")?)?,
            train_labels: read_label_file(&child_path("train-labels-idx1-ubyte")?)?,
            test_labels: read_label_file(&child_path("t10k-labels-idx1-ubyte")?)?
        })
    }

    fn train_data<'a>(&'a self) -> (&[Self::Sample], &[usize]) {
        (self.train_data.as_slice(), self.train_labels.as_slice())
    }

    fn test_data<'a>(&'a self) -> (&[Self::Sample], &[usize]) {
        (self.test_data.as_slice(), self.test_labels.as_slice())
    }
}

fn read_label_file(path: &str) -> Result<Vec<usize>, io::Error> {
    let mut f = File::open(path)?;
    let mut field32 = [0u8; 4];
    f.read_exact(&mut field32)?;
    if field32 != [0u8, 0u8, 8u8, 1u8] {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "unexpected magic number"));
    }
    f.read_exact(&mut field32)?;
    let count = (field32[3] as usize) + ((field32[2] as usize) << 8);
    let mut result = Vec::new();
    f.read_to_end(&mut result)?;
    if result.len() != count {
        Err(io::Error::new(io::ErrorKind::InvalidData, "unexpected number of entries"))
    } else {
        Ok(result.into_iter().map(|x| x as usize).collect())
    }
}

fn read_image_file(path: &str) -> Result<Vec<Image>, io::Error> {
    let mut f = File::open(path)?;
    let mut field32 = [0u8; 4];
    f.read_exact(&mut field32)?;
    if field32 != [0u8, 0u8, 8u8, 3u8] {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "unexpected magic number"));
    }
    f.read_exact(&mut field32)?;
    let count = (field32[3] as usize) + ((field32[2] as usize) << 8);
    for _ in 0..2 {
        f.read_exact(&mut field32)?;
        if field32 != [0u8, 0u8, 0u8, 28u8] {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "unexpected magic number"));
        }
    }
    let mut raw_result = Vec::new();
    f.read_to_end(&mut raw_result)?;
    if raw_result.len() != count * IMAGE_SIZE {
        Err(io::Error::new(io::ErrorKind::InvalidData, "unexpected number of entries"))
    } else {
        let mut result = Vec::new();
        for i in 0..count {
            let offset = i * IMAGE_SIZE;
            let mut image = Image{0: [0u8; IMAGE_SIZE]};
            for j in 0..(IMAGE_SIZE) {
                image.0[j] = raw_result[offset + j];
            }
            result.push(image);
        }
        Ok(result)
    }
}
