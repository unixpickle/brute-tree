extern crate brute_tree;
extern crate clap;
extern crate futures;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;

use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use std::sync::mpsc::{Sender, channel};
use std::thread;

use clap::{Arg, App};
use futures::future::Future;
use futures::stream::Stream;
use hyper::{Body, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

use brute_tree::evaluate::TreeEvaluation;

struct StatusService {
    sender: Sender<TreeEvaluation>
}

impl Service for StatusService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Response, Error=hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        if req.path() != "/tree" {
            return Box::new(futures::future::ok(not_found_response()));
        }
        let sender = self.sender.clone();
        Box::new(req.body().concat2().and_then(move |data| -> Result<Response, hyper::Error> {
            if let Ok(body_str) = String::from_utf8(data.into_iter().collect()) {
                let body: Result<TreeEvaluation, serde_json::Error> =
                    serde_json::from_str(&body_str);
                match body {
                    Ok(evaluation) => {
                        sender.send(evaluation).unwrap();
                        Ok(message_response("processed request"))
                    },
                    Err(_) => Ok(bad_req_response())
                }
            } else {
                Ok(bad_req_response())
            }
        }))
    }
}

fn main() {
    let matches = App::new("brute-tree-server")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Set the port to listen on")
            .takes_value(true))
        .arg(Arg::with_name("out")
            .short("o")
            .long("out")
            .value_name("FILE")
            .help("Set the path to store intermediate results")
            .takes_value(true))
        .get_matches();
    let port = matches.value_of("port").unwrap_or("1337").parse::<u16>().unwrap();
    let out_path = matches.value_of("out").unwrap_or("output.json");
    serve_on_port(port, String::from(out_path));
}

fn serve_on_port(port: u16, out_path: String) {
    let (sender, receiver) = channel::<TreeEvaluation>();
    thread::spawn(move || {
        let mut best_acc = -1f64;
        if let Some(eval) = load_from_path(&out_path) {
            println!("using accuracy threshold from existing file");
            best_acc = eval.accuracy;
        }
        let mut count = 0usize;
        while let Ok(status) = receiver.recv() {
            count += 1;
            if status.accuracy > best_acc {
                best_acc = status.accuracy;
                println!("count {}: got new best accuracy: {}", count, status.accuracy);
                let mut f = File::create(out_path.clone()).unwrap();
                f.write_all(serde_json::to_string(&status).unwrap().as_bytes()).unwrap();
                f.flush().unwrap();
            }
        }
    });

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port));
    Http::new().bind(&addr, move || {
        Ok(StatusService{sender: sender.clone()})
    }).unwrap().run().unwrap();
}

fn load_from_path(path: &str) -> Option<TreeEvaluation> {
    if let Ok(mut file) = File::open(path.clone()) {
        let mut body = String::new();
        if let Ok(_) = file.read_to_string(&mut body) {
            let parsed: Result<TreeEvaluation, serde_json::Error> = serde_json::from_str(&body);
            if let Ok(eval) = parsed {
                Some(eval)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn not_found_response() -> Response {
    Response::new().with_status(StatusCode::NotFound)
}

fn bad_req_response() -> Response {
    Response::new().with_status(StatusCode::BadRequest)
}

fn message_response(content: &str) -> Response {
    Response::new()
        .with_header(ContentLength(content.len() as u64))
        .with_body(Body::from(String::from(content)))
}
