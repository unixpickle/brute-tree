use std::cmp::PartialOrd;
use std::hash::Hash;
use std::ops::Index;
use std::collections::HashMap;

use tree::Tree;

/// A structure indicating the accuracy of a tree.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TreeEvaluation {
    pub tree: Tree,
    pub accuracy: f64
}

/// Get the maximum number of correct classifications for
/// the given tree.
pub fn evaluate<S, L>(t: &Tree, samples: &[S], labels: &[L]) -> usize
    where S: Index<usize>, S::Output: PartialOrd<u8>, L: Copy + Hash + Eq
{
    let mut mapping: HashMap<Vec<bool>, HashMap<L, usize>> = HashMap::new();
    let mut path = Vec::new();
    for (sample, label) in samples.iter().zip(labels.iter()) {
        path.clear();
        t.decision_path(&mut path, sample);
        if !mapping.contains_key(&path) {
            let mut new_counts = HashMap::<L, usize>::new();
            new_counts.insert(*label, 1);
            mapping.insert(path.clone(), new_counts);
        } else {
            let mut mapping_ref = &mut mapping;
            let mut counts = mapping_ref.get_mut(&path).unwrap();
            let new_count = if counts.contains_key(label) {
                counts[label] + 1
            } else {
                1
            };
            counts.insert(*label, new_count);
        }
    }
    mapping.values().map(|x| x.values().max().unwrap()).sum()
}
