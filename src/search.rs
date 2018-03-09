extern crate rand;

use std::cmp::PartialOrd;
use std::iter::FromIterator;
use std::ops::Index;

use rand::thread_rng;
use rand::distributions::{Range, IndependentSample};

use tree::{Tree, Branch};
use evaluate::evaluate;

pub struct Searcher {
    pub trials_per_depth: usize,
    pub mutate_prob: f64,
    pub feature_max: usize,
    pub threshold_max: u8
}

impl Searcher {
    /// Search for an improvement on the given tree.
    pub fn search<S: Index<usize>>(&self, t: &Tree, samples: &[S], labels: &[usize])
        -> (Tree, usize)
        where S::Output: PartialOrd<u8>
    {
        self.recursive_search(&t, &Vec::from_iter(samples.iter().map(|x| x)), labels,
            self.trials_per_depth)
    }

    fn recursive_search<S: Index<usize>>(&self, t: &Tree, samples: &[&S], labels: &[usize],
        trials: usize) -> (Tree, usize)
        where S::Output: PartialOrd<u8>
    {
        if !t.branch.is_none() {
            let child_res = self.child_search(t, samples, labels, trials);
            let random_res = self.random_search(&child_res.0, samples, labels, trials);
            better_tree(child_res, random_res)
        } else {
            self.random_search(t, samples, labels, trials)
        }
    }

    fn child_search<S: Index<usize>>(&self, t: &Tree, samples: &[&S], labels: &[usize],
        trials: usize) -> (Tree, usize)
        where S::Output: PartialOrd<u8>
    {
        let branch = t.branch.as_ref().unwrap();
        let mut left_samples = Vec::new();
        let mut left_labels = Vec::new();
        let mut right_samples = Vec::new();
        let mut right_labels = Vec::new();
        for (sample, label) in samples.iter().zip(labels) {
            if sample[t.feature] <= t.threshold {
                left_samples.push(*sample);
                left_labels.push(*label);
            } else {
                right_samples.push(*sample);
                right_labels.push(*label);
            }
        }
        let (left_best, left_correct) = self.recursive_search(&branch.left, &left_samples,
            &left_labels, trials);
        let (right_best, right_correct) = self.recursive_search(&branch.right, &right_samples,
            &right_labels, trials);
        (Tree{
            feature: t.feature,
            threshold: t.threshold,
            branch: Some(Box::new(Branch{left: left_best, right: right_best}))
        }, left_correct + right_correct)
    }

    fn random_search<S: Index<usize>>(&self, t: &Tree, samples: &[&S], labels: &[usize],
        trials: usize) -> (Tree, usize)
        where S::Output: PartialOrd<u8>
    {
        let correct = evaluate(t, samples, labels);
        let mut res = (t.clone(), correct);
        for _ in 0..trials {
            let mutated = self.mutate(t, true);
            let sub_correct = evaluate(&mutated, samples, labels);
            res = better_tree(res, (mutated, sub_correct));
        }
        res
    }

    fn mutate(&self, t: &Tree, root: bool) -> Tree {
        let mut_prob = if root {
            1f64
        } else {
            self.mutate_prob
        };
        let mut rng = thread_rng();
        let range = Range::new(0f64, 1f64);
        Tree{
            feature: if range.ind_sample(&mut rng) < mut_prob {
                Range::new(0usize, self.feature_max).ind_sample(&mut rng)
            } else {
                t.feature
            },
            threshold: if range.ind_sample(&mut rng) < mut_prob {
                Range::new(0u8, self.threshold_max).ind_sample(&mut rng)
            } else {
                t.threshold
            },
            branch: match &t.branch {
                &Some(ref branch) => {
                    Some(Box::new(Branch{
                        left: self.mutate(&branch.left, false),
                        right: self.mutate(&branch.right, false)
                    }))
                },
                &None => None
            }
        }
    }
}

fn better_tree(x: (Tree, usize), y: (Tree, usize)) -> (Tree, usize) {
    if x.1 < y.1 {
        y
    } else {
        x
    }
}
