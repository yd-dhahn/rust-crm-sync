extern crate base64;
extern crate chrono;
extern crate crossbeam_channel;
extern crate fallible_iterator;
extern crate futures;
extern crate hyper;
extern crate postgres;
extern crate pretty_env_logger;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate reqwest;
extern crate serde;
extern crate serde_aux;
extern crate sha1;
extern crate time;
extern crate tokio;
extern crate tokio_tungstenite;
extern crate url;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod config;
mod db;
mod salesforce;
mod server;
mod sync;
mod util;
use config::Config;
use server::ApiServer as Server;
use std::env;
use sync::Sync;

lazy_static! {
    static ref CONF: Config = Config::new("config.json").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{}", args[1]);
    if args.len() > 1 && args[1] == "-i" {
        let mut syncher = Sync::new(&CONF);
        syncher.run();
    } else {
        Server::start(&CONF);
    }
}
