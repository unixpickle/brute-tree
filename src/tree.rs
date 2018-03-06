use std::cmp::PartialOrd;
use std::ops::Index;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
/// A decision tree that is stored on the heap.
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
    /// Construct the "zero" tree of a given depth.
    pub fn new(depth: u8) -> Tree {
        Tree::last(depth, 0, 0)
    }

    /// Construct the maximum tree before wrapping occurs.
    pub fn last(depth: u8, feature_max: usize, threshold_max: u8) -> Tree {
        if depth == 0 {
            Tree{feature: feature_max, threshold: threshold_max, branch: None}
        } else {
            Tree{
                feature: feature_max,
                threshold: threshold_max,
                branch: Some(Box::new(Branch{
                    left: Tree::last(depth - 1, feature_max, threshold_max),
                    right: Tree::last(depth - 1, feature_max, threshold_max)
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

fn modular_add(start: usize, add: usize, max: usize) -> (usize, usize) {
    ((start + add) % (max + 1), (start + add) / (max + 1))
}

#[test]
fn tree_add() {
    let t1 = Tree::new(2);

    // Leftmost leaf becomes (feature=1, thresh=2)
    let (t2, t2_carry) = t1.add(5, 1, 2);
    assert_eq!(t2_carry, 0);
    assert_eq!(t2, Tree{
        feature: 0,
        threshold: 0,
        branch: Some(Box::new(Branch{
            left: Tree{
                feature: 0,
                threshold: 0,
                branch: Some(Box::new(Branch{
                    left: Tree{
                        feature: 1,
                        threshold: 2,
                        branch: None
                    },
                    right: Tree::new(0)
                }))
            },
            right: Tree::new(1)
        }))
    });

    // Leftmost leaf wraps and becomes (feature=0, thresh=2).
    // Second to leftmost leaf becomes (feature=0, thresh=1).
    let (t3, t3_carry) = t2.add(3, 1, 2);
    assert_eq!(t3_carry, 0);
    assert_eq!(t3, Tree{
        feature: 0,
        threshold: 0,
        branch: Some(Box::new(Branch{
            left: Tree{
                feature: 0,
                threshold: 0,
                branch: Some(Box::new(Branch{
                    left: Tree{
                        feature: 0,
                        threshold: 2,
                        branch: None
                    },
                    right: Tree{
                        feature: 0,
                        threshold: 1,
                        branch: None
                    }
                }))
            },
            right: Tree::new(1)
        }))
    });

    // Leaf nodes have 6 combos, depth 1 has 6^3 combos,
    // depth 2 has (6^3)^2*6 = 279936 combos.
    let (t4, t4_carry) = t3.add(279935, 1, 2);
    assert_eq!(t4_carry, 1);
    assert_eq!(t4, Tree{
        feature: 0,
        threshold: 0,
        branch: Some(Box::new(Branch{
            left: Tree{
                feature: 0,
                threshold: 0,
                branch: Some(Box::new(Branch{
                    left: Tree{
                        feature: 0,
                        threshold: 1,
                        branch: None
                    },
                    right: Tree{
                        feature: 0,
                        threshold: 1,
                        branch: None
                    }
                }))
            },
            right: Tree::new(1)
        }))
    });

    let (t5, t5_carry) = t3.add(279936 - 8, 1, 2);
    assert_eq!(t5_carry, 1);
    assert_eq!(t5, Tree::new(2));

    let (t6, t6_carry) = t3.add(279936 - 9, 1, 2);
    assert_eq!(t6_carry, 0);
    assert_eq!(t6, Tree::last(2, 1, 2));
}
