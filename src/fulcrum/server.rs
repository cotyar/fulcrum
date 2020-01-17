#![warn(dead_code)]
#![warn(unused_imports)]


// pub mod pb {
//     tonic::include_proto!("fulcrum");
// }

use tracing::{debug, error, Level};
// use tracing_subscriber::FmtSubscriber;
use tracing_attributes::instrument;
// use tracing_futures;

// use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::collections::VecDeque;

use prost::Message;
use sled::{Config as SledConfig};
use bytes::{Buf, IntoBuf};

// use futures::Stream;
use std::fmt;
use std::net::SocketAddr;
// use std::pin::Pin;
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status /*, Streaming*/};

use pb::*;
use pb::cdn_control_server::*;
use pb::cdn_query_server::*;

use internal_error::{*, Cause::*};

mod data_access;
use data_access::*;
use data_access::pb as pb;

mod cdn;
use cdn::*;

use sled::Db;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::Subscriber::builder()
        // all spans/events with a level higher than DEBUG (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // .with_env_filter("attrs_basic=trace")
        // sets this to be the default, global subscriber for this application.
        .init();

    let addrs = ["[::1]:50151", "[::1]:50152"];

    let (tx, mut rx) = mpsc::unbounded_channel();

    let config = SledConfig::new().temporary(true);
    let db = config.open()?;

    for addr in &addrs {
        let addr = addr.parse()?;
        let tx = tx.clone();

        let control_server = CdnServer { addr, db: db.clone() };
        let query_server = CdnServer { addr, db: db.clone() };
        let serve = Server::builder()
            .add_service(pb::cdn_control_server::CdnControlServer::new(control_server))
            .add_service(pb::cdn_query_server::CdnQueryServer::new(query_server))
            .serve(addr);

        tokio::spawn(async move {
            if let Err(e) = serve.await {
                eprintln!("Error = {:?}", e);
            }

            tx.send(()).unwrap();
        });
    }

    rx.recv().await;

    Ok(())
}