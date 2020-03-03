pub mod pb {
    tonic::include_proto!("fulcrum");
}

//use pb::{echo_client::EchoClient, EchoRequest};
use tonic::transport::Channel;

mod data_access;
use crate::data_access::*;

#[async_trait]
pub trait DataTree where Self: Uid {
    async fn put<U: Clone + Send + fmt::Debug +'static>(self: &Self, value: U) -> Result<U, Box<dyn std::error::Error>>;
    
    async fn get_page_by_prefix<U: Clone + Send + fmt::Debug +'static> 
        (tree: &Tree, buffer_size: usize, key: Option<Self>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, 
            f: Box<dyn Fn((sled::IVec, sled::IVec)) -> PageResult<U> + Send>)
            -> Result<Receiver<PageResult<U>>, InternalError>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoints = ["http://[::1]:50051", "http://[::1]:50052"]
        .iter()
        .map(|a| Channel::from_static(a));

    let channel = Channel::balance_list(endpoints);

    // let mut client = EchoClient::new(channel);

    // for _ in 0..12usize {
    //     let request = tonic::Request::new(EchoRequest {
    //         message: "hello".into(),
    //     });

    //     let response = client.unary_echo(request).await?;

    //     println!("RESPONSE={:?}", response);
    // }

    Ok(())
}