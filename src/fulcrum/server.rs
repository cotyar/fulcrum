#![warn(dead_code)]
#![warn(unused_imports)]


pub mod pb {
    tonic::include_proto!("fulcrum");
}

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

use sled::Db;

type GrpcResult<T> = Result<Response<T>, Status>;
// type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send + Sync>>;

#[derive(Debug)]
pub struct CdnServer {
    addr: SocketAddr,
    db: Db
}

impl fmt::Display for CdnUid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CdnUid: {}", self.message)
    }
}

use internal_error::{*, Cause::*};

trait ProstMessageExt<T: ::prost::Message + Default> {
    fn to_bytes(self: &Self) -> Result<Vec<u8>, InternalError>;
    fn from_bytes<B: Buf>(msg_bytes: B) -> Result<T, InternalError>;
}

impl<T: ::prost::Message + Default> ProstMessageExt<T> for T {
    fn to_bytes(self: &Self) -> Result<Vec<u8>, InternalError> { 
        let mut msg_bytes = Vec::new();
        self.encode(&mut msg_bytes)
            .map_err(|e|
                InternalError { cause: Some(StorageValueEncodingError(
                    EncodeError { required: e.required_capacity() as u64, remaining: e.remaining() as u64 } )) })?;
        Ok(msg_bytes)
    }

    fn from_bytes<B: Buf>(msg_bytes: B) -> Result<Self, InternalError> {
        let v = Self::decode(msg_bytes)
            .map_err(|e| {
                let ee = Box::new(e) as Box<dyn std::error::Error>;
                InternalError { cause: Some(StorageValueDecodingError(
                    DecodeError { description: ee.to_string(), stack: Vec::new()} )) } // TODO: Populate Stack
            })?;
        Ok(v)
    }
}

fn unwrap_field<T: ::prost::Message + Default>(msg: Option<T>, field_name: &str) -> Result<T, InternalError> { 
    msg.ok_or(InternalError { cause: Some(MissingRequiredArgument(field_name.to_string())) })
}

fn process_uid<T> (r_uid: Option<CdnUid>, f: impl FnOnce(&CdnUid, &Vec<u8>) -> Result<T, ::sled::Error>) -> Result<(CdnUid, T), InternalError> {
    let uid = unwrap_field(r_uid, "uid")?;
    let uid_bytes = uid.to_bytes()?;

    let old_value = f(&uid, &uid_bytes)
        .map_err(|e| InternalError { cause: Some(StorageError(e.to_string())) })?;
    Ok((uid, old_value))
}


// impl Hash for CdnValue {
//     fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
//         where Self: Sized
//     {
//         for piece in data {
//             piece.hash(state);
//         }
//     }
//     fn hash<H: Hasher>(&self, state: &mut H) { 
//         self.message.hash(state);
//     }
// }

#[tonic::async_trait]
impl CdnControl for CdnServer {

    #[instrument]
    async fn add(&self, request: Request<CdnAddRequest>) -> GrpcResult<CdnAddResponse> {
        let r = request.into_inner();
        debug!("Add Received: '{:?}':'{:?}' (from {})", r.uid, r.value, self.addr);        
        
        let res = || -> Result<cdn_add_response::Resp, InternalError> {
            let value = unwrap_field(r.value, "value")?;
            let value_bytes = value.to_bytes()?;

            let check_and_insert = |uid1: &CdnUid, uid_bytes1: &Vec<u8>| -> Result<_, ::sled::Error> {
                
                let contains = self.db.contains_key(uid_bytes1)?;
                if contains { 
                    Ok(cdn_add_response::Resp::Exists(()))
                }
                else {
                    let existing = self.db.insert(uid_bytes1, value_bytes)?; 
                    if existing.is_some() {
                        error!("Unexpected override of the value in store: '{}'", uid1); 
                    }
                    Ok(cdn_add_response::Resp::Success(uid1.clone()))
                } 
            };    
            
            let (_, ret) = process_uid(r.uid, check_and_insert)?;
            Ok(ret)
        };
        
        let result = match res() {
            Ok(resp) => resp,
            Err(e) => cdn_add_response::Resp::Error(e)
        };

        Ok(Response::new(CdnAddResponse { resp: Some(result) }))
    }

    async fn delete(&self, request: Request<CdnDeleteRequest>) -> GrpcResult<CdnDeleteResponse> {
        let r = request.into_inner();
        debug!("'{:?}' (from {})", r.uid, self.addr);
        
        let res = match process_uid(r.uid, |_, uid_bytes| self.db.remove(uid_bytes)) {
            Ok((uid, Some(_))) => cdn_delete_response::Resp::Success(uid),
            Ok((_, None)) => cdn_delete_response::Resp::NotFound(()), //(Status::not_found(uid.to_string())),
            Err(e) => cdn_delete_response::Resp::Error(e)
        };

        Ok(Response::new(CdnDeleteResponse { resp: Some(res) }))
    }
}


#[tonic::async_trait]
impl CdnQuery for CdnServer {

    // #[instrument(level = "debug")]
    #[instrument]
    async fn get(&self, request: Request<CdnGetRequest>) -> GrpcResult<CdnGetResponse> {
        let r = request.into_inner();
        debug!("Get Received: '{:?}' (from {})", r.uid, self.addr); // TODO: Fix tracing and remove
        
        let res = match process_uid(r.uid, |_, uid_bytes| self.db.get(uid_bytes)) {
            Ok((uid, Some(v_bytes))) => {
                match CdnValue::from_bytes(v_bytes.into_buf()) {
                    Ok(v) => cdn_get_response::Resp::Success(v),
                    Err(e) => cdn_get_response::Resp::Error(e),
                }
            },
            Ok((_, None)) => cdn_get_response::Resp::NotFound(()), //(Status::not_found(uid.to_string())),
            Err(e) => cdn_get_response::Resp::Error(e)
        };

        Ok(Response::new(CdnGetResponse { resp: Some(res) }))
    }

    async fn contains(&self, request: Request<CdnContainsRequest>) -> GrpcResult<CdnContainsResponse> {
        let r = request.into_inner();
        debug!("Contains Received: '{:?}' (from {})", r.uid, self.addr);

        let res = match process_uid(r.uid, |_, uid_bytes| self.db.contains_key(uid_bytes)) {
            Ok((_, v)) => cdn_contains_response::Resp::Success(v),
            Err(e) => cdn_contains_response::Resp::Error(e)
        };

        Ok(Response::new(CdnContainsResponse { resp: Some(res) }))
    }

    type StreamValueStream = mpsc::Receiver<Result<CdnStreamValueResponse, Status>>;
    async fn stream_value(&self, request: Request<CdnGetRequest>) -> GrpcResult<Self::StreamValueStream> {
        let r = request.into_inner();
        let message = format!("'{:?}' (from {})", r.uid, self.addr);
        println!("StreamValueStream Received: {}", message);

        let key = r.uid.clone();

        let (mut tx, rx) = mpsc::channel(4);
        let db = self.db.clone();

        type StreamValueStreamSender = mpsc::Sender<Result<CdnStreamValueResponse, Status>>;

        async fn get_kv(db: &Db, tx: &mut StreamValueStreamSender, key: &CdnUid) -> Result<Vec<CdnUid>, InternalError> {
            let res = match process_uid(Some(key.clone()), |_, uid_bytes| db.get(uid_bytes)) {
                Ok((uid, Some(v_bytes))) => {
                    let value = CdnValue::decode(v_bytes.into_buf()).unwrap();
                    let v = value.message.as_ref().unwrap();
                    match v {
                        cdn_value::Message::Batch(cdn_value::Batch { uids }) => {
                            let keys = uids.clone();
                            let resp = cdn_stream_value_response::Resp::Success(CdnKeyValue { key: Some(key.clone()), value: Some(value) });
                            let msg = Ok(CdnStreamValueResponse { resp: Some(resp) });
                            println!("StreamValueStream sending batch item: {:?}", &msg);
                            tx.send(msg).await.unwrap();
                            Ok(keys)
                        },
                        _ => {
                            let resp = cdn_stream_value_response::Resp::Success(CdnKeyValue { key: Some(key.clone()), value: Some(value) });
                            let msg = Ok(CdnStreamValueResponse { resp: Some(resp) });
                            println!("StreamValueStream sending: {:?}", &msg);
                            tx.send(msg).await.unwrap();
                            Ok(Vec::new())
                        }
                    }

                    // match CdnValue::from_bytes(v_bytes.into_buf()) {
                    //     Ok(v) => cdn_get_response::Resp::Success(v),
                    //     Err(e) => cdn_get_response::Resp::Error(e),
                    // }
                },
                Ok((_, None)) => { 
                    let resp = cdn_stream_value_response::Resp::NotFound(());
                    tx.send(Ok(CdnStreamValueResponse { resp: Some(resp) })).await.unwrap();
                    Ok(Vec::new())
                }, //cdn_get_response::Resp::NotFound(()), //(Status::not_found(uid.to_string())),
                Err(e) => { 
                    let resp = cdn_stream_value_response::Resp::Error(e);
                    tx.send(Ok(CdnStreamValueResponse { resp: Some(resp) })).await.unwrap();
                    Ok(Vec::new())
                }
            };

            res
    
            // let uid_bytes = key.to_bytes()
            //     .map_err(|s|cdn_stream_value_response::Failure {cause: Some(cdn_stream_value_response::failure::Cause::InternalError(format!("{:?}:{}", s.code(), s.message())))})?;
    
            // match db.get(&uid_bytes).unwrap() {
            //     Some(v) => { 
            //         let value = CdnValue::decode(v.into_buf()).unwrap();
            //         let v = value.message.as_ref().unwrap();
            //         match v {
            //             cdn_value::Message::Batch(cdn_value::Batch { uids }) => {
            //                 let keys = uids.clone();
            //                 let resp = cdn_stream_value_response::Resp::Success(CdnKeyValue { key: Some(key.clone()), value: Some(value) });
            //                 let msg = Ok(CdnStreamValueResponse { result: Some(resp) });
            //                 println!("StreamValueStream sending batch item: {:?}", &msg);
            //                 tx.send(msg).await.unwrap();
            //                 Ok(keys)
            //             },
            //             _ => {
            //                 let resp = cdn_stream_value_response::Resp::Success(CdnKeyValue { key: Some(key.clone()), value: Some(value) });
            //                 let msg = Ok(CdnStreamValueResponse { result: Some(resp) });
            //                 println!("StreamValueStream sending: {:?}", &msg);
            //                 tx.send(msg).await.unwrap();
            //                 Ok(Vec::new())
            //             }
            //         }
            //     },
            //     None => {
            //         let resp = cdn_stream_value_response::Resp::Error( cdn_stream_value_response::Failure { cause: Some(cdn_stream_value_response::failure::Cause::NotFound(())) });
            //         tx.send(Ok(CdnStreamValueResponse { result: Some(resp) })).await.unwrap();
            //         Ok(Vec::new())
            //     }
            // }
        }

        tokio::spawn(async move {
            let mut seen = HashSet::<String>::new(); // TODO: implement Hash for CdnUid
            let mut remaining_keys = VecDeque::new();
            remaining_keys.push_back(key.unwrap());
            
            while 
                match remaining_keys.pop_front() { 
                    Some (next_key) => {
                        let rk = get_kv(&db, &mut tx, &next_key).await;
                        for k in rk {
                            // if !seen.contains(&k.message) {
                            //     seen.insert(k.message.clone());
                            //     remaining_keys.push_back(k);
                            // }
                        }
                        true
                    },
                    None => false
                }
            {
            }
        });

        Ok(Response::new(rx))
    }
}

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