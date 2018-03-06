extern crate brute_tree;
extern crate futures;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;

use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use std::sync::mpsc::{Sender, channel};
use std::thread;

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
    // TODO: parse argument here.
    listen_on_port(1337u16);
}

fn listen_on_port(port: u16) {
    let (sender, receiver) = channel::<TreeEvaluation>();
    thread::spawn(move || {
        let mut best_acc = -1f64;
        while let Ok(status) = receiver.recv() {
            if status.accuracy > best_acc {
                best_acc = status.accuracy;
                // TODO: dump JSON tree to a file.
                println!("got new best accuracy: {}", status.accuracy);
            }
        }
    });

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port));
    Http::new().bind(&addr, move || {
        Ok(StatusService{sender: sender.clone()})
    }).unwrap().run().unwrap();
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
