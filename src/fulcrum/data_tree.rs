use tracing::{debug, error, Level};
// use tracing_subscriber::FmtSubscriber;
use tracing_attributes::instrument;
// use tracing_futures;

// use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::collections::VecDeque;

use prost::Message;
use sled::{Config as SledConfig};

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
use crate::data_tree::KeyColumn::*;

#[derive(Debug, Clone)]
pub enum KeyColumn {
    SimpleKeyColumn(Tree),
    // IndexedKeyColumn { uid_tree: Tree, index_tree: Tree } 
}

#[derive(Debug, Clone)]
pub struct DataTreeServer {
    pub addr: SocketAddr,
    pub tree: KeyColumn
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
        
        let res = match &self.tree { 
            SimpleKeyColumn(tree) => 
                match add(&tree, key_uid, r.value) { // TODO: Add override flag and/or "return previous"
                    Success(uid) => Resp::Success(uid),
                    Exists(uid) => Resp::Exists(uid), 
                    Error(e) => Resp::Error(e)
                },
            // IndexedKeyColumn { uid_tree, index_tree }  => {
            //     let key = r.key.map(|k| k.key).flatten();
            //     match add(&uid_tree, key_uid, r.value) { // TODO: Add override flag and/or "return previous"
            //         Success(uid) => {
            //             match add(&index_tree, key, r.value) { // TODO: Add override flag and/or "return previous"
            //                 Success(uid) => Resp::Success(uid),
            //                 Exists(uid) => Resp::Exists(uid), 
            //                 Error(e) => Resp::Error(e)
            //             }                        
            //         },
            //         Exists(uid) => Resp::Exists(uid), 
            //         Error(e) => Resp::Error(e)
            //     }
            // }
        };

        Ok(Response::new(AddResponse { resp: Some(res) }))
    }

    async fn copy(&self, request: Request<CopyRequest>) -> GrpcResult<CopyResponse> {
        type Resp = copy_response::Resp;

        let r = request.into_inner();
        debug!("Copy Received: from_key '{:?}' to_key: '{:?}' (from {})", r.key_from, r.key_to, self.addr); // TODO: Fix tracing and remove
        
        let r_for_resp = r.clone(); 
        let res = match &self.tree { 
            SimpleKeyColumn(tree) => 
                match get::<_, Entry>(&tree, r.key_from) {
                    GetResult::Success(_uid, v) => {
                        match add(&tree, r.key_to, Some(v)) { // TODO: Add override flag and/or "return previous"
                            AddResult::Success(_uid) => Resp::Success(r_for_resp),
                            AddResult::Exists(_uid) => Resp::ToKeyExists(r_for_resp), 
                            AddResult::Error(e) => Resp::Error(e)
                        }      
                    },
                    GetResult::NotFound(_uid) => Resp::FromKeyNotFound(r_for_resp), 
                    GetResult::Error(e) => Resp::Error(e)
                }
        };

        debug!("Copy Response: '{:?}'", res); 
        Ok(Response::new(CopyResponse { resp: Some(res) }))
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> GrpcResult<DeleteResponse> {
        use DeleteResult::*;
        type Resp = delete_response::Resp;

        let r = request.into_inner();
        debug!("'{:?}' (from {})", r.uid, self.addr);
        
        let res = match &self.tree { 
            SimpleKeyColumn(tree) => 
                match delete(&tree, r.uid) {
                    Success(uid) => Resp::Success(uid),
                    NotFound(uid) => Resp::NotFound(uid), 
                    Error(e) => Resp::Error(e)
                }
        };

        Ok(Response::new(DeleteResponse { resp: Some(res) }))
    }

    async fn get(&self, request: Request<GetRequest>) -> GrpcResult<GetResponse> {
        use GetResult::*;
        type Resp = get_response::Resp;

        let r = request.into_inner();
        debug!("Get Received: '{:?}' (from {})", r.uid, self.addr); // TODO: Fix tracing and remove
        
        let res = match &self.tree { 
            SimpleKeyColumn(tree) => 
                match get(&tree, r.uid) {
                    Success(uid, v) => Resp::Success(v),
                    NotFound(uid) => Resp::NotFound(uid), 
                    Error(e) => Resp::Error(e)
                }
        };

        debug!("Get Response: '{:?}'", res); 
        Ok(Response::new(GetResponse { resp: Some(res) }))
    }

    async fn contains(&self, request: Request<ContainsRequest>) -> GrpcResult<ContainsResponse> {
        let r = request.into_inner();
        debug!("Contains Received: '{:?}' (from {})", r.uid, self.addr);

        let res = match &self.tree { 
            SimpleKeyColumn(tree) => 
                match contains_key(&tree, r.uid) {
                    Ok(v) => contains_response::Resp::Success(v),
                    Err(e) => contains_response::Resp::Error(e)
                }
        };

        Ok(Response::new(ContainsResponse { resp: Some(res) }))
    }

    #[doc = "Server streaming response type for the SearchKeys method."]
    type SearchStream = mpsc::Receiver<Result<SearchResponse, Status>>;
    async fn search(&self, request: Request<SearchRequest>) -> GrpcResult<Self::SearchStream> {
        let r = request.into_inner();
        debug!("Search Received: '{:?}' (from {})", r, self.addr);

        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}