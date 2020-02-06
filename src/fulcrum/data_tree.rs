use tracing::{debug, error, Level};
// use tracing_subscriber::FmtSubscriber;
use tracing_attributes::instrument;
// use tracing_futures;

// use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

use bytes::{Bytes, Buf, BufMut};

use prost::Message;
use sled::{Config as SledConfig};

// use futures::Stream;
use std::fmt;
use std::net::SocketAddr;
// use std::pin::Pin;
use tokio::sync::mpsc;
use tokio::stream::StreamExt;
use tonic::{transport::Server, Request, Response, Status /*, Streaming*/};

use crate::data_access::*;
use crate::pb::*;

use internal_error::{*, Cause::*};

use sled::{Tree};
use data_tree_server::DataTree;
use crate::data_tree::KeyColumn::*;

extern crate siphasher;

use siphasher::sip::{SipHasher24};

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

fn hasher (k: KeyString) -> Vec<u8> {
    let mut h = SipHasher24::new();
    k.hash(&mut h);
    h.finish().to_be_bytes().to_vec()
}

fn to_key_uid (k: KeyString) -> KeyUid {
    KeyUid { sip: hasher(k) }
}

#[tonic::async_trait]
impl DataTree for DataTreeServer {
    async fn add(&self, request: Request<AddRequest>) -> GrpcResult<AddResponse> {
        use AddResult::*;
        type Resp = add_response::Resp;

        let r = request.into_inner();
        debug!("Add Received: '{:?}':'{:?}' (from {})", r.key, r.value, self.addr);        
        
        // let key_uid = r.key.map(|k| k.uid).flatten();
        // let (key_uid, key) = r.key.map(|k| (k.uid, k.key));
        let key = r.key.map(|k| KeyString(k.key));
        
        let res = match &self.tree { 
            SimpleKeyColumn(tree) => 
                match add(&tree, key, r.value) { // TODO: Add override flag and/or "return previous"
                    Success(k) => Resp::Success(to_key_uid(k)),
                    Exists(k) => Resp::Exists(to_key_uid(k)), 
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
        let key_from = r.key_from.map(|k| KeyString(k.key));
        let key_to = r.key_to.map(|k| KeyString(k.key));

        let res = match &self.tree { 
            SimpleKeyColumn(tree) => 
                match get::<_, Entry>(&tree, key_from) {
                    GetResult::Success(_uid, v) => {
                        match add(&tree, key_to, Some(v)) { // TODO: Add override flag and/or "return previous"
                            AddResult::Success(_k) => Resp::Success(r_for_resp),
                            AddResult::Exists(_k) => Resp::ToKeyExists(r_for_resp), 
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
        debug!("'{:?}' (from {})", r.key, self.addr);

        let key = r.key.map(|k| KeyString(k.key));
        
        let res = match &self.tree { 
            SimpleKeyColumn(tree) => 
                match delete(&tree, key) {
                    Success(k) => Resp::Success(to_key_uid(k)),
                    NotFound(k) => Resp::NotFound(to_key_uid(k)), 
                    Error(e) => Resp::Error(e)
                }
        };

        Ok(Response::new(DeleteResponse { resp: Some(res) }))
    }

    async fn get(&self, request: Request<GetRequest>) -> GrpcResult<GetResponse> {
        use GetResult::*;
        type Resp = get_response::Resp;

        let r = request.into_inner();
        debug!("Get Received: '{:?}' (from {})", r.key, self.addr); // TODO: Fix tracing and remove

        let key = r.key.map(|k| KeyString(k.key));
        
        let res = match &self.tree { 
            SimpleKeyColumn(tree) => 
                match get(&tree, key) {
                    Success(_k, v) => Resp::Success(v),
                    NotFound(k) => Resp::NotFound(to_key_uid(k)), 
                    Error(e) => Resp::Error(e)
                }
        };

        debug!("Get Response: '{:?}'", res); 
        Ok(Response::new(GetResponse { resp: Some(res) }))
    }

    async fn contains(&self, request: Request<ContainsRequest>) -> GrpcResult<ContainsResponse> {
        let r = request.into_inner();
        debug!("Contains Received: '{:?}' (from {})", r.key, self.addr);

        let key = r.key.map(|k| KeyString(k.key));

        let res = match &self.tree { 
            SimpleKeyColumn(tree) => 
                match contains_key(&tree, key) {
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
        type Resp = search_response::Resp;
        debug!("Search Received: '{:?}' (from {})", r, self.addr);

        let include_value = r.include_value;
        let result_mapper: Box<dyn Fn((sled::IVec, sled::IVec)) -> PageResult<(KeyString, Option<Entry>)> + Send> = 
            Box::new(move |(k_bytes, v_bytes)| {
                match Uid::from_key_bytes(&*k_bytes) {
                    Ok(key) => {
                        if include_value {
                            match Entry::from_bytes(Bytes::from(v_bytes.to_vec())) {
                                Ok(v) => PageResult::Success((key, Some(v))),
                                Err(e) => PageResult::ValueError(e),
                            }
                        } 
                        else {
                            PageResult::Success((key, None))
                        }
                    },
                    Err(e) => PageResult::KeyError(e)
                }
            });

        let res = match &self.tree { 
            SimpleKeyColumn(tree) => 
                get_page_by_prefix_str(tree, 100, Some(KeyString(r.key_prefix)), Some(r.page), Some(r.page_size), 
                    1000, result_mapper).await            
        };

        match res {
            Ok(rxi) => {
                let (mut tx, rx) = mpsc::channel(100);
                tokio::spawn(async move { 
                    let mut rxx = rxi;
                    loop {
                        let page_result = rxx.recv().await;
                        debug!("Sending page result: {:?}", page_result.clone());
                        match page_result {
                            Some(PageResult::Success::<_>(page_res)) => {
                                debug!("Sending mapped key: {:?}", page_res.clone());
                                let (k, v) = page_res;
                                let item = SearchResponseItem { key: Some(Key { key: k.0.clone(), key_family: None, uid: None /*Some(to_key_uid(k))*/ }), value: v, metadata: None };
                                let resp = Resp::Success(item);
                                match tx.send(Ok(SearchResponse { resp: Some(resp.clone()) })).await {
                                    Ok(()) => (),
                                    Err(e) => {
                                        error!("Error streaming response: '{:?}', '{:?}'", resp, e);
                                        break;
                                    }
                                };
                            },
                            Some(PageResult::KeyError(e)) => {
                                let resp = Resp::KeyError(e);
                                match tx.send(Ok(SearchResponse { resp: Some(resp.clone()) })).await {
                                    Ok(()) => (),
                                    Err(e) => {
                                        error!("Error streaming response: '{:?}', '{:?}'", resp, e);
                                        break;
                                    }
                                };
                            },
                            Some(PageResult::ValueError(e)) => {
                                let resp = Resp::ValueError(e);
                                match tx.send(Ok(SearchResponse { resp: Some(resp.clone()) })).await {
                                    Ok(()) => (),
                                    Err(e) => {
                                        error!("Error streaming response: '{:?}', '{:?}'", resp, e);
                                        break;
                                    }
                                };
                            },
                            None => break
                        }
                    }
                }); 
                Ok(Response::new(rx))    
            },
            Err(e) => Err(Status::out_of_range(format!("Error: {:?}", e)))
        }
    }
}