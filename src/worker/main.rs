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
use brute_tree::evolve::{best_eval, next_generation};

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
        .arg(Arg::with_name("population")
            .short("p")
            .long("population")
            .value_name("INT")
            .help("Set the population size")
            .takes_value(true))
        .arg(Arg::with_name("elite")
            .short("e")
            .long("elite")
            .value_name("INT")
            .help("Set the number of selected individuals")
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
    let population = matches.value_of("population").unwrap_or("100").parse::<usize>().unwrap();
    let elite = matches.value_of("elite").unwrap_or("10").parse::<usize>().unwrap();
    let restart = matches.value_of("restart").unwrap_or("100").parse::<usize>().unwrap();
    let noise = matches.value_of("noise").unwrap_or("0.02").parse::<f64>().unwrap();

    worker_loop(dataset, server_url, depth, population, elite, restart, noise).unwrap();
}

fn worker_loop<D: Dataset>(dataset: D, server_url: &str, depth: u8, population: usize,
    elite: usize, restart: usize, noise: f64) -> Result<(), hyper::Error>
    where <<D as Dataset>::Sample as std::ops::Index<usize>>::Output: std::cmp::PartialOrd<u8>
{
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::new(&handle);

    let (samples, labels) = dataset.train_data();

    loop {
        println!("doing random restart");
        let mut trees = Vec::new();
        for _ in 0..population {
            trees.push(Tree::random(depth, D::feature_max(), D::threshold_max()));
        }
        let mut best_ever = 0f64;
        let mut stagnate_iters = 0;
        while stagnate_iters < restart {
            let start_time = Instant::now();
            let evals: Vec<TreeEvaluation> = (&trees).into_iter().map(|t| {
                let num_correct = evaluate(t, samples, labels);
                TreeEvaluation{
                    tree: t.clone(),
                    accuracy: (num_correct as f64) / (samples.len() as f64)
                }
            }).collect();

            let time_per_tree = start_time.elapsed() / (trees.len() as u32);
            println!("best_acc={:.3} tree_time={}.{:04}", best_eval(&evals).accuracy,
                time_per_tree.as_secs(), time_per_tree.subsec_nanos() / 100000);

            let best = best_eval(&evals);
            if best.accuracy > best_ever {
                stagnate_iters = 0;
                best_ever = best.accuracy;
                send_result(&mut core, &client, server_url, best)?;
            } else {
                stagnate_iters += 1;
            }

            trees = next_generation(&evals, elite, noise, D::feature_max(), D::threshold_max());
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
