use std::cmp::PartialOrd;
use std::ops::Index;
use rand::thread_rng;
// TODO: figure out why no Distributions trait (must be an old version).
use rand::distributions::{IndependentSample, Range};

/// A decision tree that is stored on the heap.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Tree {
    feature: usize,
    threshold: u8,
    branch: Option<Box<Branch>>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Branch {
    left: Tree,
    right: Tree
}

impl Tree {
    /// Construct a random tree of the given depth.
    pub fn random(depth: u8, features: &Range<usize>, thresholds: &Range<u8>) -> Tree
    {
        let mut rng = thread_rng();
        let feature = features.ind_sample(&mut rng);
        let threshold = thresholds.ind_sample(&mut rng);
        if depth == 0 {
            Tree{feature: feature, threshold: threshold, branch: None}
        } else {
            Tree{
                feature: feature,
                threshold: threshold,
                branch: Some(Box::new(Branch{
                    left: Tree::random(depth - 1, features, thresholds),
                    right: Tree::random(depth - 1, features, thresholds)
                })),
            }
        }
    }

    /// Get the decision path for the feature map.
    pub fn decision_path<T: Index<usize>>(&self, path: &mut Vec<bool>, sample: &T)
        where T::Output: PartialOrd<u8>
    {
        let right = sample[self.feature] > self.threshold;
        path.push(right);
        if let &Some(ref branch) = &self.branch {
            (if right {
                &branch.right
            } else {
                &branch.left
            }).decision_path(path, sample);
        }
    }
}
