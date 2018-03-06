#[derive(Clone)]
pub struct HeapBranch {
    left: HeapTree,
    right: HeapTree
}

#[derive(Clone)]
/// A decision tree that is stored on the heap.
pub struct HeapTree {
    feature: u32,
    threshold: u8,
    branch: Option<Box<HeapBranch>>
}

impl HeapTree {
    /// Construct the "zero" tree of a given depth.
    pub fn new(depth: u8) -> HeapTree {
        if depth == 0 {
            HeapTree{feature: 0, threshold: 0, branch: None}
        } else {
            HeapTree{
                feature: 0,
                threshold: 0,
                branch: Some(Box::new(HeapBranch{
                    left: HeapTree::new(depth - 1),
                    right: HeapTree::new(depth - 1)
                }))
            }
        }
    }

    /// Increment the tree under a well-defined ordering.
    /// If the increment causes wrap-around, the second
    /// return value is true.
    pub fn next_tree(&self, feature_max: u32, threshold_max: u8) -> (HeapTree, bool) {
        if let &Some(ref branch) = &self.branch {
            let (new_left, wrap_left) = branch.left.next_tree(feature_max, threshold_max);
            let (new_right, wrap_right) = if wrap_left {
                branch.right.next_tree(feature_max, threshold_max)
            } else {
                (branch.right.clone(), false)
            };
            let stump = HeapTree{feature: self.feature, threshold: self.threshold, branch: None};
            let (new_stump, wrap) = if wrap_right {
                stump.next_tree(feature_max, threshold_max)
            } else {
                (stump, false)
            };
            (HeapTree{
                feature: new_stump.feature,
                threshold: new_stump.threshold,
                branch: Some(Box::new(HeapBranch{left: new_left, right: new_right}))
            }, wrap)
        } else {
            if self.threshold < threshold_max {
                (HeapTree{feature: self.feature, threshold: self.threshold + 1, branch: None},
                    false)
            } else if self.feature < feature_max {
                (HeapTree{feature: self.feature+1, threshold: 0, branch: None}, false)
            } else {
                (HeapTree{feature: 0, threshold: 0, branch: None}, true)
            }
        }
    }
}
