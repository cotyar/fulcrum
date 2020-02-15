use tracing::{debug, error};
use tracing_attributes::instrument;
// use tracing_futures;

use std::net::SocketAddr;
// use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::collections::VecDeque;

use sled::{Tree, TransactionError};

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

        let r = request.into_inner();
        debug!("Add Received: '{:?}':'{:?}' (from {})", r.uid, r.value, self.addr);        
        
        let add_result: &Result<Resp, TransactionError<InternalError>> = &self.tree.transaction(move |tree| {
            match add(tree, r.uid.clone(), r.value.clone()) {
                Ok(AddResultSuccess::Success(uid)) => Ok(Resp::Success(uid)),
                Ok(AddResultSuccess::Exists(uid)) => Ok(Resp::Exists(uid)), 
                Err(e) => Ok(Resp::Error(e))
            }
        });
        let add_res: Result<Resp, InternalError> = add_result.clone().map_err(|e| e.into());

        let res = match add_res {
            Ok(r) => r,
            Err(e) => Resp::Error(e)
        };

        Ok(Response::new(CdnAddResponse { resp: Some(res) }))
    }

    async fn delete(&self, request: Request<CdnDeleteRequest>) -> GrpcResult<CdnDeleteResponse> {
        type Resp = cdn_delete_response::Resp;

        let r = request.into_inner();
        debug!("'{:?}' (from {})", r.uid, self.addr);
        
        let res = match delete(&self.tree, r.uid) {
            Ok(DeleteResultSuccess::Success(uid)) => Resp::Success(uid),
            Ok(DeleteResultSuccess::NotFound(uid)) => Resp::NotFound(uid), 
            Err(e) => Resp::Error(e)
        };

        Ok(Response::new(CdnDeleteResponse { resp: Some(res) }))
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
        
        let res = match get(&self.tree, r.uid) {
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

        let res = match contains_key(&self.tree, r.uid) {
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

        let key = r.uid;

        let (mut tx, rx) = mpsc::channel(100); // TODO: Move constant to config
        let tree = self.tree.clone();

        async fn get_kv(tree: &Tree, tx: &mut StreamValueStreamSender, key: Option<CdnUid>) -> Vec<CdnUid> {
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
                                remaining_keys.push_back(Some(k));
                            }
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