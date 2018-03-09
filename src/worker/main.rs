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
use brute_tree::evaluate::TreeEvaluation;
use brute_tree::dataset::Dataset;
use brute_tree::dataset::mnist::MNIST;
use brute_tree::search::{Searcher};

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
        .arg(Arg::with_name("depth")
            .short("d")
            .long("depth")
            .value_name("INT")
            .help("Set the tree depth")
            .takes_value(true))
        .arg(Arg::with_name("trials")
            .short("t")
            .long("trials")
            .value_name("INT")
            .help("Set the number of trials per depth")
            .takes_value(true))
        .arg(Arg::with_name("restart")
            .short("r")
            .long("restart")
            .value_name("INT")
            .help("Set the number of generations of no improvement before a restart")
            .takes_value(true))
        .arg(Arg::with_name("noise")
            .short("n")
            .long("noise")
            .value_name("FLOAT")
            .help("Set the mutate probability")
            .takes_value(true))
        .get_matches();

    let server_url = matches.value_of("server").unwrap_or("http://localhost:1337/tree");
    let mnist_dir = matches.value_of("mnist").unwrap_or("mnist_dir");
    let dataset = MNIST::load(mnist_dir).expect("failed to load MNIST");
    let depth = matches.value_of("depth").unwrap_or("5").parse::<u8>().unwrap();
    let trials = matches.value_of("trials").unwrap_or("64").parse::<usize>().unwrap();
    let restart = matches.value_of("restart").unwrap_or("100").parse::<usize>().unwrap();
    let noise = matches.value_of("noise").unwrap_or("0.02").parse::<f64>().unwrap();

    worker_loop(dataset, server_url, depth, restart, Searcher{
        trials_per_depth: trials,
        mutate_prob: noise,
        feature_max: MNIST::feature_max(),
        threshold_max: MNIST::threshold_max()
    }).unwrap();
}

fn worker_loop<D: Dataset>(dataset: D, server_url: &str, depth: u8, restart: usize,
    searcher: Searcher) -> Result<(), hyper::Error>
    where <<D as Dataset>::Sample as std::ops::Index<usize>>::Output: std::cmp::PartialOrd<u8>
{
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::new(&handle);

    let (samples, labels) = dataset.train_data();

    loop {
        println!("doing random restart");
        let mut tree = Tree::random(depth, D::feature_max(), D::threshold_max());
        let mut best_ever = 0f64;
        let mut stagnate_iters = 0;
        while stagnate_iters < restart {
            let start_time = Instant::now();
            let (new_tree, correct) = searcher.search(&tree, samples, labels);
            tree = new_tree;
            let eval = TreeEvaluation{
                tree: tree.clone(),
                accuracy: (correct as f64) / (samples.len() as f64)
            };

            let total_time = start_time.elapsed();
            println!("best_acc={:.5} search_time={}.{:03}", eval.accuracy, total_time.as_secs(),
                total_time.subsec_nanos() / 1000000);

            if eval.accuracy > best_ever {
                stagnate_iters = 0;
                best_ever = eval.accuracy;
                send_result(&mut core, &client, server_url, &eval)?;
            } else {
                stagnate_iters += 1;
            }
        }
    }
}

fn send_result<T>(core: &mut Core, client: &Client<T>, url: &str, result: &TreeEvaluation)
    -> Result<(), hyper::Error> where T: hyper::client::Connect
{
    let uri = url.parse()?;
    let mut request = Request::new(Method::Post, uri);
    request.set_body(serde_json::to_string(result).unwrap());
    core.run(client.request(request))?;
    Ok(())
}
