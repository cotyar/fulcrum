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

use crate::data_access::*;
use crate::pb::*;

use internal_error::{*, Cause::*};

use sled::{Tree};
use data_tree_server::DataTree;

#[derive(Debug, Clone)]
pub struct DataTreeServer {
    pub addr: SocketAddr,
    pub tree: Tree
}

type GrpcResult<T> = Result<Response<T>, Status>;
// type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send + Sync>>;

#[tonic::async_trait]
impl DataTree for DataTreeServer {
    async fn add(&self, request: Request<AddRequest>) -> GrpcResult<AddResponse> {
        use AddResult::*;
        type Resp = add_response::Resp;

        let r = request.into_inner();
        debug!("Add Received: '{:?}':'{:?}' (from {})", r.key, r.value, self.addr);        
        
        let key_uid = r.key.map(|k| k.uid).flatten();        
        
        let res = match add(&self.tree, key_uid, r.value) { // TODO: Add override flag and/or "return previous"
            Success(uid) => Resp::Success(uid),
            Exists(uid) => Resp::Exists(uid), 
            Error(e) => Resp::Error(e)
        };

        Ok(Response::new(AddResponse { resp: Some(res) }))
    }

    async fn copy(&self, request: Request<CopyRequest>) -> GrpcResult<CopyResponse> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> GrpcResult<DeleteResponse> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn get(&self, request: Request<GetRequest>) -> GrpcResult<GetResponse> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn contains(&self, request: Request<ContainsRequest>) -> GrpcResult<ContainsResponse> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    #[doc = "Server streaming response type for the SearchKeys method."]
    type SearchKeysStream = mpsc::Receiver<Result<SearchKeyResponse, Status>>;
    async fn search_keys(&self, request: Request<GetRequest>) -> GrpcResult<Self::SearchKeysStream> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    
    #[doc = "Server streaming response type for the SearchKeyValues method."]
    type SearchKeyValuesStream = mpsc::Receiver<Result<SearchKeyValueResponse, Status>>;
    async fn search_key_values(&self, request: Request<GetRequest>) -> GrpcResult<Self::SearchKeyValuesStream> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}