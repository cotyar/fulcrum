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
use crate::pb::cdn_control_server::*;
use crate::pb::cdn_query_server::*;

use internal_error::{*, Cause::*};

use sled::Db;
use data_tree_server::DataTree;


type GrpcResult<T> = Result<Response<T>, Status>;
// type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send + Sync>>;

#[tonic::async_trait]
impl DataTree for FulcrumServer {
    async fn add(
        &self,
        request: tonic::Request<super::AddRequest>,
    ) -> Result<tonic::Response<super::AddResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn copy(
        &self,
        request: tonic::Request<super::CopyRequest>,
    ) -> Result<tonic::Response<super::CopyResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn delete(
        &self,
        request: tonic::Request<super::DeleteRequest>,
    ) -> Result<tonic::Response<super::DeleteResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn get(
        &self,
        request: tonic::Request<super::GetRequest>,
    ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn contains(
        &self,
        request: tonic::Request<super::ContainsRequest>,
    ) -> Result<tonic::Response<super::ContainsResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    #[doc = "Server streaming response type for the SearchKeys method."]
    type SearchKeysStream: Stream<Item = Result<super::SearchKeyResponse, tonic::Status>>
        + Send
        + Sync
        + 'static;
    async fn search_keys(
        &self,
        request: tonic::Request<super::GetRequest>,
    ) -> Result<tonic::Response<Self::SearchKeysStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    #[doc = "Server streaming response type for the SearchKeyValues method."]
    type SearchKeyValuesStream: Stream<Item = Result<super::SearchKeyValueResponse, tonic::Status>>
        + Send
        + Sync
        + 'static;
    async fn search_key_values(
        &self,
        request: tonic::Request<super::GetRequest>,
    ) -> Result<tonic::Response<Self::SearchKeyValuesStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}