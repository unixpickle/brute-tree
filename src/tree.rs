use std::cmp::PartialOrd;
use std::ops::Index;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
/// A decision tree that is stored on the heap.
pub struct Tree {
    feature: usize,
    threshold: u8,
    branch: Option<Box<Branch>>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
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

    /// Advance forward by n trees.
    ///
    /// The "remainder" is yielded if wraparound occurs.
    /// In this case, it represents the number of times
    /// the add wrapped around.
    pub fn add(&self, n: usize, feature_max: usize, threshold_max: u8) -> (Tree, usize) {
        if let &Some(ref branch) = &self.branch {
            let (new_left, left_carry) = branch.left.add(n, feature_max, threshold_max);
            let (new_right, right_carry) = branch.right.add(left_carry, feature_max,
                threshold_max);
            let stump = Tree{feature: self.feature, threshold: self.threshold, branch: None};
            let (new_stump, stump_carry) = stump.add(right_carry, feature_max, threshold_max);
            (Tree{
                feature: new_stump.feature,
                threshold: new_stump.threshold,
                branch: Some(Box::new(Branch{left: new_left, right: new_right}))
            }, stump_carry)
        } else {
            let (new_thresh, thresh_carry) = modular_add(self.threshold as usize, n,
                threshold_max as usize);
            let (new_feature, feature_carry) = modular_add(self.feature, thresh_carry,
                feature_max);
            (Tree{feature: new_feature, threshold: new_thresh as u8, branch: None}, feature_carry)
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

fn modular_add(start: usize, add: usize, limit: usize) -> (usize, usize) {
    ((start + add) % limit, (start + add) / limit)
}
