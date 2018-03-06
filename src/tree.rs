use std::cmp::PartialOrd;
use std::ops::Index;

#[derive(Clone)]
/// A decision tree that is stored on the heap.
pub struct Tree {
    feature: usize,
    threshold: u8,
    branch: Option<Box<Branch>>
}

#[derive(Clone)]
pub struct Branch {
    left: Tree,
    right: Tree
}

impl Tree {
    /// Construct the "zero" tree of a given depth.
    pub fn new(depth: u8) -> Tree {
        if depth == 0 {
            Tree{feature: 0, threshold: 0, branch: None}
        } else {
            Tree{
                feature: 0,
                threshold: 0,
                branch: Some(Box::new(Branch{
                    left: Tree::new(depth - 1),
                    right: Tree::new(depth - 1)
                }))
            }
        }
    }

    /// Increment the tree under a well-defined ordering.
    /// If the increment causes wrap-around, the second
    /// return value is true.
    pub fn next_tree(&self, feature_max: usize, threshold_max: u8) -> (Tree, bool) {
        if let &Some(ref branch) = &self.branch {
            let (new_left, wrap_left) = branch.left.next_tree(feature_max, threshold_max);
            let (new_right, wrap_right) = if wrap_left {
                branch.right.next_tree(feature_max, threshold_max)
            } else {
                (branch.right.clone(), false)
            };
            let stump = Tree{feature: self.feature, threshold: self.threshold, branch: None};
            let (new_stump, wrap) = if wrap_right {
                stump.next_tree(feature_max, threshold_max)
            } else {
                (stump, false)
            };
            (Tree{
                feature: new_stump.feature,
                threshold: new_stump.threshold,
                branch: Some(Box::new(Branch{left: new_left, right: new_right}))
            }, wrap)
        } else {
            if self.threshold < threshold_max {
                (Tree{feature: self.feature, threshold: self.threshold + 1, branch: None},
                    false)
            } else if self.feature < feature_max {
                (Tree{feature: self.feature+1, threshold: 0, branch: None}, false)
            } else {
                (Tree{feature: 0, threshold: 0, branch: None}, true)
            }
        }
    }

    /// Get the decision path for the feature map.
    pub fn decision_path<T: Index<usize>>(&self, path: &mut Vec<bool>, sample: T)
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
