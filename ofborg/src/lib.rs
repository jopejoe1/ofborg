
#![recursion_limit="512"]

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate serde_json;

#[macro_use]
extern crate log;

#[macro_use]
extern crate nom;

extern crate hubcaps;
extern crate hyper;
extern crate hyper_native_tls;
extern crate either;
extern crate lru_cache;
extern crate tempfile;
extern crate amqp;
extern crate fs2;
extern crate md5;
extern crate uuid;
extern crate env_logger;
extern crate chrono;

use std::env;

pub mod acl;
pub mod checkout;
pub mod locks;
pub mod clone;
pub mod worker;
pub mod config;
pub mod message;
pub mod tasks;
pub mod evalchecker;
pub mod files;
pub mod nix;
pub mod stats;
pub mod ghevent;
pub mod commentparser;
pub mod commitstatus;
pub mod outpathdiff;
pub mod tagger;
pub mod asynccmd;
pub mod notifyworker;
pub mod writetoline;
pub mod test_scratch;
pub mod easyamqp;

pub mod ofborg {
    pub use asynccmd;
    pub use stats;
    pub use config;
    pub use checkout;
    pub use locks;
    pub use clone;
    pub use worker;
    pub use notifyworker;
    pub use message;
    pub use tasks;
    pub use evalchecker;
    pub use files;
    pub use commitstatus;
    pub use ghevent;
    pub use nix;
    pub use acl;
    pub use commentparser;
    pub use outpathdiff;
    pub use tagger;
    pub use writetoline;
    pub use test_scratch;
    pub use easyamqp;

    pub const VERSION: &str = env!("CARGO_PKG_VERSION");

    pub fn partition_result<A,B>(results: Vec<Result<A,B>>) -> (Vec<A>, Vec<B>) {
        let mut ok = Vec::new();
        let mut err = Vec::new();
        for result in results.into_iter() {
            match result {
                Ok(x) => { ok.push(x); }
                Err(x) => { err.push(x); }
            }
        }

        (ok, err)
    }
}

pub fn setup_log() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
        env_logger::init().unwrap();
        info!("Defaulting RUST_LOG environment variable to info");
    } else {
        env_logger::init().unwrap();
    }
}
