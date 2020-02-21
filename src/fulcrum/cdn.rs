use crate::error_handling::unwrap_field;
use tracing::{debug, error};
use tracing_attributes::instrument;
// use tracing_futures;

use std::net::SocketAddr;
// use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::collections::VecDeque;

use sled::{Tree};

use tokio::sync::mpsc;
use tonic::{Request, Response, Status /*, Streaming*/};

use crate::data_access::*;
use crate::pb::*;
use crate::pb::cdn_control_server::*;
use crate::pb::cdn_query_server::*;

use internal_error::{*, Cause::*};


type GrpcResult<T> = Result<Response<T>, Status>;
// type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send + Sync>>;


#[derive(Debug, Clone)]
pub struct CdnServer {
    pub addr: SocketAddr,
    pub tree: Tree
}

#[tonic::async_trait]
impl CdnControl for CdnServer {

    #[instrument]
    async fn add(&self, request: Request<CdnAddRequest>) -> GrpcResult<CdnAddResponse> {
        type Resp = cdn_add_response::Resp;

        let r: CdnAddRequest = request.into_inner();
        debug!("Add Received: '{:?}':'{:?}' (from {})", r.uid, r.value, self.addr);        

        let res = || {
            let uid: CdnUid = unwrap_field(r.uid, "uid")?;
            let value = unwrap_field(r.value, "value")?;
            
            self.tree.transaction::<_,_,InternalError>(|tree| {
                match add(tree, uid.clone(), value.clone()) { // TODO: Add override flag and/or "return previous"
                    Ok(AddResultSuccess::Success(uid)) => Ok(Resp::Success(uid)),
                    Ok(AddResultSuccess::Exists(uid)) => Ok(Resp::Exists(uid)), 
                    Err(e) => Ok(Resp::Error(e))
                }
            }).map_err(|e| InternalError::from(e))
        };

        let resp = match res() {
            Ok(r) => r,
            Err(e) => Resp::Error(e)
        }; 

        Ok(Response::new(CdnAddResponse { resp: Some(resp) }))
    }

    async fn delete(&self, request: Request<CdnDeleteRequest>) -> GrpcResult<CdnDeleteResponse> {
        type Resp = cdn_delete_response::Resp;

        let r = request.into_inner();
        debug!("'{:?}' (from {})", r.uid, self.addr);
        
        let res = || {
            let uid: CdnUid = unwrap_field(r.uid, "uid")?;
            
            self.tree.transaction::<_,_,InternalError>(|tree| {
                match delete(tree, uid.clone()) {
                    Ok(DeleteResultSuccess::Success(uidd)) => Ok(Resp::Success(uidd)),
                    Ok(DeleteResultSuccess::NotFound(uidd)) => Ok(Resp::NotFound(uidd)), 
                    Err(e) => Ok(Resp::Error(e))
                }
            }).map_err(|e| InternalError::from(e))
        };

        let resp = match res() {
            Ok(r) => r,
            Err(e) => Resp::Error(e)
        }; 


        Ok(Response::new(CdnDeleteResponse { resp: Some(resp) }))
    }
}


type StreamValueStreamSender = mpsc::Sender<Result<CdnStreamValueResponse, Status>>;

//#[instrument]
async fn send_response_msg (tx: &mut StreamValueStreamSender, resp: cdn_stream_value_response::Resp) {
    let msg = Ok(CdnStreamValueResponse { resp: Some(resp) });
    debug!("StreamValueStream sending: {:?}", &msg);
    match tx.send(msg).await {
        Ok(()) => (),
        Err(e) => error!("Value message transfer failed with: {}", e)
    }
}

#[tonic::async_trait]
impl CdnQuery for CdnServer {

    // #[instrument(level = "debug")]
    #[instrument]
    async fn get(&self, request: Request<CdnGetRequest>) -> GrpcResult<CdnGetResponse> {
        use GetResultSuccess::*;
        type Resp = cdn_get_response::Resp;

        let r = request.into_inner();
        debug!("Get Received: '{:?}' (from {})", r.uid, self.addr); // TODO: Fix tracing and remove
        
        let res = match unwrap_field(r.uid, "uid").and_then(|uid| get(&self.tree, uid)) {
            Ok(Success(_uid, v)) => Resp::Success(v),
            Ok(NotFound(uid)) => Resp::NotFound(uid), 
            Err(e) => Resp::Error(e)
        };

        debug!("Get Response: '{:?}'", res); 
        Ok(Response::new(CdnGetResponse { resp: Some(res) }))
    }

    async fn contains(&self, request: Request<CdnContainsRequest>) -> GrpcResult<CdnContainsResponse> {
        let r = request.into_inner();
        debug!("Contains Received: '{:?}' (from {})", r.uid, self.addr);

        let contains_res = unwrap_field(r.uid, "uid").and_then(|uid| contains_key(&self.tree, uid));
        let res = match contains_res {
            Ok(v) => cdn_contains_response::Resp::Success(v),
            Err(e) => cdn_contains_response::Resp::Error(e)
        };

        Ok(Response::new(CdnContainsResponse { resp: Some(res) }))
    }

    type StreamValueStream = mpsc::Receiver<Result<CdnStreamValueResponse, Status>>;
    async fn stream_value(&self, request: Request<CdnGetRequest>) -> GrpcResult<Self::StreamValueStream> {
        type Resp = cdn_stream_value_response::Resp;

        let r = request.into_inner();
        let message = format!("'{:?}' (from {})", r.uid, self.addr);
        println!("StreamValueStream Received: {}", message);

        let (mut tx, rx) = mpsc::channel(100); // TODO: Move constant to config

        match r.uid {
            None => return Err(Status::invalid_argument(String::from("uid field is missing"))),
            Some(key) => {
                let tree = self.tree.clone();

                async fn get_kv(tree: &Tree, tx: &mut StreamValueStreamSender, key: CdnUid) -> Vec<CdnUid> {
                    match get::<CdnUid, CdnValue>(tree, key) {
                        Ok(GetResultSuccess::Success(uid, v)) => {
                            let msg = &v.message;
                            match msg {
                                Some(cdn_value::Message::Batch(cdn_value::Batch { uids })) => {
                                    send_response_msg(tx, Resp::Success(CdnKeyValue { key: Some(uid), value: Some(v.clone()) })).await;
                                    uids.clone()
                                },
                                Some(cdn_value::Message::Bytes(_)) => {
                                    send_response_msg(tx, Resp::Success(CdnKeyValue { key: Some(uid), value: Some(v.clone()) })).await;
                                    Vec::new()
                                },
                                None => {
                                    let e = InternalError { cause: Some(StorageValueDecodingError( DecodeError { description: "'message' field is required".to_string(), stack: vec![ internal_error::decode_error::StackLine { message: "field is required".to_string(), field: "message".to_string() } ] })) };
                                    error! ("uid: {}, '{:?}'", uid, e);
                                    send_response_msg(tx, Resp::Error(e)).await;
                                    Vec::new()
                                }
                            }
                        },
                        Ok(GetResultSuccess::NotFound(uid)) => { 
                            send_response_msg(tx, Resp::NotFound(uid)).await;
                            Vec::new()
                        }, 
                        Err(e) => {
                            send_response_msg(tx, Resp::Error(e)).await;
                            Vec::new()
                        }
                    }
                }

                tokio::spawn(async move {
                    let mut seen = HashSet::<String>::new(); // TODO: implement Hash for CdnUid
                    let mut remaining_keys = VecDeque::new();
                    remaining_keys.push_back(key);
                    
                    while 
                        match remaining_keys.pop_front() { 
                            Some (next_key) => {
                                let keys = get_kv(&tree, &mut tx, next_key).await;
                                for k in keys {
                                    if !seen.contains(&k.message) {
                                        seen.insert(k.message.clone());
                                        remaining_keys.push_back(k);
                                    }
                                }
                                true
                            },
                            None => false
                        }
                    {
                    }
                });
            }
        }

        Ok(Response::new(rx))
    }
}