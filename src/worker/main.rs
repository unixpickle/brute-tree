extern crate brute_tree;
extern crate clap;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;

use std::time::Instant;

use clap::{Arg, App};
use hyper::{Method, Request};
use hyper::client::Client;
use tokio_core::reactor::Core;

use brute_tree::tree::Tree;
use brute_tree::evaluate::{TreeEvaluation, evaluate};
use brute_tree::dataset::Dataset;
use brute_tree::dataset::mnist::MNIST;

fn main() {
    let matches = App::new("brute-tree-server")
        .arg(Arg::with_name("mnist")
            .short("m")
            .long("mnist")
            .value_name("DIR")
            .help("Set the MNIST data directory")
            .takes_value(true))
        .arg(Arg::with_name("server")
            .short("s")
            .long("server")
            .value_name("URL")
            .help("Set the URL where results are POSTed")
            .takes_value(true))
        .arg(Arg::with_name("trials")
            .short("t")
            .long("trials")
            .value_name("INT")
            .help("Set the number of trials per POST")
            .takes_value(true))
        .arg(Arg::with_name("depth")
            .short("d")
            .long("depth")
            .value_name("INT")
            .help("Set the tree depth")
            .takes_value(true))
        .get_matches();

    let server_url = matches.value_of("server").unwrap_or("http://localhost:1337/tree");
    let mnist_dir = matches.value_of("mnist").unwrap_or("mnist_dir");
    let trial_count = matches.value_of("trials").unwrap_or("10000").parse::<usize>().unwrap();
    let dataset = MNIST::load(mnist_dir).expect("failed to load MNIST");
    let depth = matches.value_of("depth").unwrap_or("5").parse::<u8>().unwrap();

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
        let start_time = Instant::now();
        for _ in 0..trial_count {
            let tree = Tree::random(depth, D::feature_max(), D::threshold_max());
            let num_correct = evaluate(&tree, samples, labels);
            if num_correct >= best_correct {
                best_correct = num_correct;
                best_tree = Some(tree);
            }
        }
        if let Some(tree) = best_tree {
            let time_per_tree = start_time.elapsed() / (trial_count as u32);
            println!("averaged {}.{:03} sec/tree", time_per_tree.as_secs(),
                time_per_tree.subsec_nanos() / 1000000);
            let result = TreeEvaluation{
                tree: tree,
                accuracy: (best_correct as f64) / (samples.len() as f64)
            };
            println!("sending result with accuracy {}", result.accuracy);
            send_result(&mut core, &client, server_url, result)?;
        }
    }
}

fn send_result<T>(core: &mut Core, client: &Client<T>, url: &str, result: TreeEvaluation)
    -> Result<(), hyper::Error> where T: hyper::client::Connect
{
    let uri = url.parse()?;
    let mut request = Request::new(Method::Post, uri);
    request.set_body(serde_json::to_string(&result).unwrap());
    core.run(client.request(request))?;
    Ok(())
}
