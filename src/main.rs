extern crate tiny_http;
extern crate chrono;

use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

use tiny_http::{Server, Response};
use chrono::Local;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("\n\tSyntax: {} listen_ip:listen_port", &args[0]);
        std::process::exit(1);
    }

    let mut logfile = match OpenOptions::new().create(true).append(true).open("desir.log") {
        Err(why) => panic!("unable to open desir.log, reason:{}", why),
        Ok(file) => file,
    };

    write!(logfile, "[{}] Starting Desir. Written by Sebastien Tricaud\n", Local::now()).ok();
    let server = Server::http(&args[1]).unwrap();

    for request in server.incoming_requests() {
        write!(logfile, "[{:?}] from={:?} {:?} {:?} {:?}\n", Local::now(), request.remote_addr(), request.method(), request.url(), request.headers()).ok();
        
        let response = Response::from_string("");
        request.respond(response).ok();
    }
    
}
