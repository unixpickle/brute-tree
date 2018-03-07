extern crate rand;

use std::cmp::PartialOrd;

use rand::thread_rng;
use rand::distributions::{Range, IndependentSample};

use tree::{Tree, Branch};
use evaluate::TreeEvaluation;

/// Get the best tree in a population.
pub fn best_eval<'a>(pop: &'a [TreeEvaluation]) -> &'a TreeEvaluation {
    pop.iter().fold(&pop[0], |eval, new_eval| {
        if eval.accuracy < new_eval.accuracy {
            new_eval
        } else {
            eval
        }
    })
}

/// Create a new generation by mutating the best trees in
/// the previous generation.
pub fn next_generation(pop: &[TreeEvaluation], elite_size: usize, prob: f64, feature_max: usize,
    threshold_max: u8) -> Vec<Tree>
{
    assert!(elite_size <= pop.len());

    let mut sorted = Vec::from(pop);
    sorted.sort_unstable_by(|x, y| y.accuracy.partial_cmp(&x.accuracy).unwrap());

    let mut result = Vec::new();
    result.push((&sorted[0]).tree.clone()); // "Elitism"
    for i in 0..(pop.len() - 1) {
        let parent = &sorted[i % elite_size];
        result.push(mutate(&parent.tree, prob, feature_max, threshold_max));
    }
    result
}

/// Apply a random mutation to the tree.
pub fn mutate(t: &Tree, prob: f64, feature_max: usize, threshold_max: u8) -> Tree {
    let mut rng = thread_rng();
    let range = Range::new(0f64, 1f64);
    Tree{
        feature: if range.ind_sample(&mut rng) < prob {
            Range::new(0usize, feature_max).ind_sample(&mut rng)
        } else {
            t.feature
        },
        threshold: if range.ind_sample(&mut rng) < prob {
            Range::new(0u8, threshold_max).ind_sample(&mut rng)
        } else {
            t.threshold
        },
        branch: match &t.branch {
            &Some(ref branch) => {
                Some(Box::new(Branch{
                    left: mutate(&branch.left, prob, feature_max, threshold_max),
                    right: mutate(&branch.right, prob, feature_max, threshold_max)
                }))
            },
            &None => None
        }
    }
}
