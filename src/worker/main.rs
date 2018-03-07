extern crate brute_tree;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;

use hyper::{Method, Request};
use hyper::client::Client;
use tokio_core::reactor::Core;

use brute_tree::tree::Tree;
use brute_tree::evaluate::{TreeEvaluation, evaluate};
use brute_tree::dataset::Dataset;
use brute_tree::dataset::mnist::MNIST;

fn main() {
    // TODO: parse command-line arguments here.
    let dataset = MNIST::load("mnist_dir").expect("failed to load MNIST");
    let server_url = "http://localhost:1337";
    let trial_count = 10000usize;
    let depth = 5u8;

    worker_loop(dataset, server_url, trial_count, depth).unwrap();
}

fn worker_loop<D: Dataset>(dataset: D, server_url: &str, trial_count: usize, depth: u8)
    -> Result<(), hyper::Error>
    where <<D as Dataset>::Sample as std::ops::Index<usize>>::Output: std::cmp::PartialOrd<u8>
{
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::new(&handle);

    let (samples, labels) = dataset.train_data();

    loop {
        let mut best_correct = 0usize;
        let mut best_tree: Option<Tree> = None;
        for i in 0..trial_count {
            let tree = Tree::random(depth, D::feature_max(), D::threshold_max());
            let num_correct = evaluate(&tree, samples, labels);
            if num_correct >= best_correct {
                best_correct = num_correct;
                best_tree = Some(tree);
            }
        }
        if let Some(tree) = best_tree {
            let result = TreeEvaluation{
                tree: tree,
                accuracy: (best_correct as f64) / (samples.len() as f64)
            };
            send_result(&mut core, &client, server_url, result)?;
        }
    }
}

fn send_result<T>(core: &mut Core, client: &Client<T>, url: &str, result: TreeEvaluation)
    -> Result<(), hyper::Error> where T: hyper::client::Connect
{
    let uri = url.parse()?;
    let mut request = Request::new(Method::Get, uri);
    request.set_body(serde_json::to_string(&result).unwrap());
    core.run(client.request(request))?;
    Ok(())
}
