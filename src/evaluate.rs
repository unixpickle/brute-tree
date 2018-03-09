use std::cmp::PartialOrd;
use std::ops::Index;

use tree::Tree;

/// A structure indicating the accuracy of a tree.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TreeEvaluation {
    pub tree: Tree,
    pub accuracy: f64
}

/// Get the maximum number of correct classifications for
/// the given tree.
///
/// The tree must be compatible with Tree::decision_path().
pub fn evaluate<S>(t: &Tree, samples: &[&S], labels: &[usize]) -> usize
    where S: Index<usize>, S::Output: PartialOrd<u8>
{
    let mut class_counts: Vec<Vec<usize>> = Vec::new();
    let max_label = *labels.into_iter().max().unwrap_or(&0usize);
    for _ in 0..t.count_decision_paths() {
        let mut counts = Vec::new();
        for _ in 0..(max_label + 1) {
            counts.push(0usize);
        }
        class_counts.push(counts);
    }
    for (sample, label) in samples.iter().zip(labels.iter()) {
        let path = t.decision_path(*sample);
        let mut counts = &mut class_counts[path];
        counts[*label] += 1;
    }
    class_counts.into_iter().map(|x| x.into_iter().max().unwrap_or(0usize)).sum()
}
