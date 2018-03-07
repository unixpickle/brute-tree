use std::cmp::PartialOrd;
use std::ops::Index;
use rand::thread_rng;
// TODO: figure out why no Distributions trait (must be an old version).
use rand::distributions::{IndependentSample, Range};

/// A decision tree that is stored on the heap.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Tree {
    pub feature: usize,
    pub threshold: u8,
    pub branch: Option<Box<Branch>>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Branch {
    pub left: Tree,
    pub right: Tree
}

impl Tree {
    /// Construct a random tree of the given depth.
    pub fn random(depth: u8, feature_max: usize, threshold_max: u8) -> Tree
    {
        let mut rng = thread_rng();
        let feature = Range::new(0usize, feature_max + 1).ind_sample(&mut rng);
        let threshold = Range::new(0usize, (threshold_max as usize) + 1)
            .ind_sample(&mut rng) as u8;
        if depth == 0 {
            Tree{feature: feature, threshold: threshold, branch: None}
        } else {
            Tree{
                feature: feature,
                threshold: threshold,
                branch: Some(Box::new(Branch{
                    left: Tree::random(depth - 1, feature_max, threshold_max),
                    right: Tree::random(depth - 1, feature_max, threshold_max)
                })),
            }
        }
    }

    /// Get the number of possible decision paths.
    ///
    /// This only works for trees compatible with
    /// Tree::decision_path().
    pub fn count_decision_paths(&self) -> usize {
        if let &Some(ref branch) = &self.branch {
            let child_paths = branch.left.count_decision_paths();
            assert_eq!(branch.right.count_decision_paths(), child_paths);
            child_paths * 2
        } else {
            2
        }
    }

    /// Get the decision path for the feature map.
    ///
    /// This only produces valid outputs for certain trees.
    /// Specifically, the tree must be balanced, and it
    /// must have few enough decision paths to fit in usize.
    pub fn decision_path<T: Index<usize>>(&self, sample: &T) -> usize
        where T::Output: PartialOrd<u8>
    {
        let right = sample[self.feature] > self.threshold;
        let bit = if right {
            1usize
        } else {
            0usize
        };
        if let &Some(ref branch) = &self.branch {
            ((if right {
                &branch.right
            } else {
                &branch.left
            }).decision_path(sample) << 1) | bit
        } else {
            bit
        }
    }
}
