#![warn(dead_code)]


use tracing::{debug, error};
use std::hash::{Hash, Hasher};

use bytes::Bytes;

use std::net::SocketAddr;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status /*, Streaming*/};

use crate::data_access::*;
use crate::pb::*;

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

fn hasher (k: &KeyString) -> Vec<u8> {
    let mut h = SipHasher24::new();
    k.hash(&mut h);
    h.finish().to_be_bytes().to_vec()
}

fn to_key_uid (k: &KeyString) -> KeyUid {
    KeyUid { sip: hasher(k) }
}

fn build_kv_entry (k: Key, v: Option<ValueEntry>) -> (KvEntry, KeyString, KeyUid) {
    let key = KeyString(k.key);
    let key_uid = to_key_uid(&key);
    let kv_entry = KvEntry {
        // kv_metadata: None,
        metadata: Some (
            KvMetadata {
                key_uid: Some(key_uid.clone()),
                status: kv_metadata::Status::Active as i32, // TODO: Change proto enum-s to oneof-s
                expiry: None,
                /// VectorClock originated      = 4;
                /// VectorClock locallyUpdated  = 5;
                action: kv_metadata::UpdateAction::Added as i32,
                created_by: None,
                created_at: None,
                correlation_id: String::from("####"),
                originator_replica_id: String::from("0"),
                value_metadata: Some(ValueMetadata {
                    hashed_with: value_metadata::HashedWith::Sip as i32,
                    hash: vec!(1,2,3),
                    compression: value_metadata::Compression::None as i32,
                    size_compressed: 10,
                    size_full: 12,
                    serializer_id: String::from("TBD"),
                })
            }
        ),
        value: v
    };
    (kv_entry, key, key_uid)
}

#[tonic::async_trait]
impl DataTree for DataTreeServer {
    async fn add(&self, request: Request<AddRequest>) -> GrpcResult<AddResponse> {
        use AddResultSuccess::*;
        type Resp = add_response::Resp;

        let r = request.into_inner();
        debug!("Add Received: '{:?}':'{:?}' (from {})", r.key, r.value, self.addr);        
        
        let res = || {
            match &self.tree { 
                SimpleKeyColumn(tree) => {
                    let key = unwrap_field(r.key, "key")?;
                    let (kv_entry, key, _key_uid) = build_kv_entry(key, r.value);
                    
                    tree.transaction::<_,_,InternalError>(|tree| {
                        match add(tree, key.clone(), kv_entry.clone()) { // TODO: Add override flag and/or "return previous"
                            Ok(Success(k)) => Ok(Resp::Success(Key { key: k.0, key_family: None, uid: None })),
                            Ok(Exists(k)) => Ok(Resp::Exists(Key { key: k.0, key_family: None, uid: None })), 
                            Err(e) => Ok(Resp::Error(e))
                        }
                    }).map_err(|e| InternalError::from(e))
                },
                // IndexedKeyColumn { uid_tree, index_tree }  => {
                //     let kv = build_kv_entry(r.key, r.value);
                //     let (kv_entry, key, _key_uid) = (kv.clone().map(|(v,_,_)| v), kv.clone().map(|(_,v,_)| v), kv.map(|(_,_,v)| v));
                    
                //     let add_result: Result<_, TransactionError<InternalError>> = tree.transaction(move |tree| {
                //         match add(tree, key.clone(), kv_entry.clone()) { // TODO: Add override flag and/or "return previous"
                //             Ok(Success(k)) => Ok(Resp::Success(Key { key: k.0, key_family: None, uid: None })),
                //             Ok(Exists(k)) => Ok(Resp::Exists(Key { key: k.0, key_family: None, uid: None })), 
                //             Err(e) => Ok(Resp::Error(e))
                //         }
                //     });
                //     let add_res: Result<_, InternalError> = add_result.map_err(|e| e.into());

                //     match add_res {
                //         Ok(r) => r,
                //         Err(e) => Resp::Error(e)
                //     } 
                // }
            }
        };

        let resp: Resp = match res() {
            Ok(r) => r,
            Err(e) => Resp::Error(e)
        }; 

        Ok(Response::new(AddResponse { resp: Some(resp) }))
    }

    async fn copy(&self, request: Request<CopyRequest>) -> GrpcResult<CopyResponse> {
        type Resp = copy_response::Resp;

        let r = request.into_inner();
        debug!("Copy Received: from_key '{:?}' to_key: '{:?}' (from {})", r.key_from, r.key_to, self.addr); // TODO: Fix tracing and remove
        
        let r_for_resp = r.clone(); 

        let res = || {
            let key_from = unwrap_field(r.key_from, "key_from")?;
            let key_to = unwrap_field(r.key_to, "key_to")?;
            match &self.tree { 
                SimpleKeyColumn(tree) => 
                    match get::<_, KvEntry>(&tree, KeyString(key_from.key))? {
                        GetResultSuccess::Success(_uid, v) => {
                            tree.transaction::<_,_,InternalError>(|tree|
                                match add(&tree, KeyString(key_to.key.clone()), v.clone()) { // TODO: Add override flag and/or "return previous" // TODO: Update metadata
                                    Ok(AddResultSuccess::Success(_k)) => Ok(Resp::Success(r_for_resp.clone())),
                                    Ok(AddResultSuccess::Exists(_k)) => Ok(Resp::ToKeyExists(r_for_resp.clone())),
                                    Err(e) => Ok(Resp::Error(e))
                                }
                            ).map_err(|e| InternalError::from(e))
                        },
                        GetResultSuccess::NotFound(_uid) => Ok(Resp::FromKeyNotFound(r_for_resp))
                    }
            }
        };

        let resp: Resp = match res() {
            Ok(r) => r,
            Err(e) => Resp::Error(e)
        }; 
        
        debug!("Copy Response: '{:?}'", resp); 
        Ok(Response::new(CopyResponse { resp: Some(resp) }))
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> GrpcResult<DeleteResponse> {
        use DeleteResultSuccess::*;
        type Resp = delete_response::Resp;

        let r = request.into_inner();
        debug!("'{:?}' (from {})", r.key, self.addr);

        let res = match &self.tree { 
            SimpleKeyColumn(tree) => {
                match unwrap_field(r.key, "key").and_then(|key| delete(&tree, KeyString(key.key))) {
                    Ok(Success(k)) => Resp::Success(to_key_uid(&k)),
                    Ok(NotFound(k)) => Resp::NotFound(to_key_uid(&k)), 
                    Err(e) => Resp::Error(e)
                }
            }
        };

        Ok(Response::new(DeleteResponse { resp: Some(res) }))
    }

    async fn get(&self, request: Request<GetRequest>) -> GrpcResult<GetResponse> {
        type Resp = get_response::Resp;

        let r = request.into_inner();
        debug!("Get Received: '{:?}' (from {})", r.key, self.addr); // TODO: Fix tracing and remove

        let res = match &self.tree { 
            SimpleKeyColumn(tree) =>
                match unwrap_field(r.key, "key").and_then(|key| get(&tree, KeyString(key.key))) {
                    Ok(GetResultSuccess::Success(_k, v)) => if r.return_metadata { Resp::Success(v) } else { Resp::Success(KvEntry { metadata: None, value: v.value }) },
                    Ok(GetResultSuccess::NotFound(k)) => Resp::NotFound(to_key_uid(&k)), 
                    Err(e) => Resp::Error(e)
                }
        };

        debug!("Get Response: '{:?}'", res); 
        Ok(Response::new(GetResponse { resp: Some(res) }))
    }

    async fn contains(&self, request: Request<ContainsRequest>) -> GrpcResult<ContainsResponse> {
        let r = request.into_inner();
        debug!("Contains Received: '{:?}' (from {})", r.key, self.addr);

        let res = match &self.tree { 
            SimpleKeyColumn(tree) => 
                match unwrap_field(r.key, "key").and_then(|key| contains_key(&tree, KeyString(key.key))) {
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

        let return_value = r.return_value;
        let return_metadata = r.return_metadata; 
        let result_mapper: Box<dyn Fn((sled::IVec, sled::IVec)) -> PageResult<(KeyString, Option<KvEntry>)> + Send> = 
            Box::new(move |(k_bytes, v_bytes)| {
                let key = Uid::from_key_bytes(&*k_bytes)?;                        
                let key_entry = if return_value || return_metadata {
                    let key_entry = KvEntry::from_bytes(Bytes::from(v_bytes.to_vec()))?;
                    Some(key_entry)
                } 
                else { None };
                Ok(PageResultSuccess::Success((key, key_entry)))
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
                            Some(Ok(PageResultSuccess::Success::<_>(page_res))) => {
                                debug!("Sending mapped key: {:?}", page_res.clone());
                                let (k, v) = page_res;
                                let item = SearchResponseItem { key: Some(Key { key: k.0.clone(), key_family: None, uid: None /*Some(to_key_uid(k))*/ }), 
                                    value: if return_value { v.clone().and_then(|vv|vv.value) } else { None }, 
                                    metadata: if return_metadata { v.and_then(|vv|vv.metadata) } else { None } };
                                let resp = Resp::Success(item);
                                match tx.send(Ok(SearchResponse { resp: Some(resp.clone()) })).await {
                                    Ok(()) => (),
                                    Err(e) => {
                                        error!("Error streaming response: '{:?}', '{:?}'", resp, e);
                                        break;
                                    }
                                };
                            },
                            Some(Err(e)) => {
                                let resp = Resp::KeyError(e);
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