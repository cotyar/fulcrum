#![warn(dead_code)]
#![warn(unused_imports)]

use crate::error_handling::unwrap_field;
use core::borrow::Borrow;
use core::hash::{Hash, Hasher};
use tokio::sync::{mpsc, mpsc::Receiver};
use std::fmt;

use bytes::{Bytes, Buf};

extern crate async_trait;
use async_trait::async_trait;

use crate::pb::*;
use internal_error::{*, Cause::*};

use tracing::{debug, error};

use sled::{IVec, Tree, TransactionalTree, ConflictableTransactionError};

pub trait ProstMessage : ::prost::Message + Default {}
pub trait Uid : fmt::Debug + Send + Sync + Clone + fmt::Display {
    fn to_key_bytes(self: &Self) -> Result<Vec<u8>, InternalError>;
    fn from_key_bytes<B: Buf>(msg_bytes: B) -> Result<Self, InternalError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyU64(pub u64);

impl Uid for KeyU64 {
    fn to_key_bytes(self: &Self) -> Result<Vec<u8>, InternalError> {
        Ok(self.0.to_be_bytes().to_vec())
    }
    fn from_key_bytes<B: Buf>(mut msg_bytes: B) -> Result<Self, InternalError> {
        Ok(KeyU64(msg_bytes.get_u64()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyString(pub String);

impl Hash for KeyString {
    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H) where Self: Sized
    {
        for piece in data {
            state.write(piece.0.as_bytes());
        }
    }
    fn hash<H: Hasher>(&self, state: &mut H) { 
        state.write(self.0.as_bytes());
    }
}

impl Uid for KeyString {
    fn to_key_bytes(self: &Self) -> Result<Vec<u8>, InternalError> {
        Ok(self.0.bytes().collect())
    }
    fn from_key_bytes<B: Buf>(msg_bytes: B) -> Result<Self, InternalError> {
        String::from_utf8(msg_bytes.bytes().iter().cloned().collect()).
            map_err(|e| InternalError { cause: Some(StorageValueDecodingError(DecodeError{ description: e.to_string(), stack: Vec::new() }))}).
            map(|s| KeyString(s))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyVec(Vec<u8>);

impl Uid for KeyVec {
    fn to_key_bytes(self: &Self) -> Result<Vec<u8>, InternalError> {
        Ok(self.0.clone())
    }
    fn from_key_bytes<B: Buf>(msg_bytes: B) -> Result<KeyVec, InternalError> {
        Ok(KeyVec(msg_bytes.bytes().iter().cloned().collect()))
    }
}

impl Eq for CdnUid {} 
impl ProstMessage for CdnUid {} 
impl Uid for CdnUid {
    fn to_key_bytes(self: &Self) -> Result<Vec<u8>, InternalError> {
        self.to_bytes()
    }
    fn from_key_bytes<B: Buf>(msg_bytes: B) -> Result<CdnUid, InternalError> {
        CdnUid::from_bytes(msg_bytes)
    }
}

impl Eq for KeyUid {} 
impl ProstMessage for KeyUid {}
impl Uid for KeyUid {
    fn to_key_bytes(self: &Self) -> Result<Vec<u8>, InternalError> {
        self.to_bytes()
    }
    fn from_key_bytes<B: Buf>(msg_bytes: B) -> Result<KeyUid, InternalError> {
        KeyUid::from_bytes(msg_bytes)
    }
} // TODO: Revise Key structure

impl ProstMessage for CdnValue {}
impl ProstMessage for ValueEntry {}
impl ProstMessage for KvEntry {}

impl fmt::Display for CdnUid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Uid: {}", self.message)
    }
}

impl fmt::Display for KeyUid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KeyUid: {:?}", self)
    }
}

impl fmt::Display for KeyU64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KeyU64: {:?}", self)
    }
}

impl fmt::Display for KeyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KeyString: {:?}", self)
    }
}


impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key: {}, KeyUid: {:?}, KeyFamily: {:?}", self.key, self.uid, self.key_family)
    }
}

impl fmt::Display for KeyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
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

pub trait ProstMessageExt where Self: Sized {
    fn to_bytes(self: &Self) -> Result<Vec<u8>, InternalError>;
    fn from_bytes<B: Buf>(msg_bytes: B) -> Result<Self, InternalError>;
}

impl<T: ProstMessage> ProstMessageExt for T {
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

#[derive(Debug)]
pub enum GetResultSuccess<K: Uid + Borrow<K>, U: ProstMessage> {
    Success (K, U),
    NotFound (K),
}

pub type GetResult<K, U> = std::result::Result<GetResultSuccess<K, U>, InternalError>;

pub fn get<K: Uid + Borrow<K>, U: ProstMessage> (tree: &Tree, key: K) -> GetResult<K, U> {
    let key_bytes = key.to_key_bytes()?;
    let v_bytes_opt = tree.get(key_bytes)?;
    match v_bytes_opt {
        Some (v_bytes) => {
            let bts = v_bytes.to_vec();
            let v = U::from_bytes(Bytes::from(bts))?;
            Ok(GetResultSuccess::Success(key, v))
        },
        None => Ok(GetResultSuccess::NotFound(key))
    }
}

pub fn get_transactional<K: Uid + Borrow<K>, U: ProstMessage> (tree: &TransactionalTree, key: K) -> GetResult<K, U> {
    let key_bytes = key.to_key_bytes()?;
    let v_bytes_opt = tree.get(key_bytes)    
        .map_err(ConflictableTransactionError::<Option<IVec>>::from)
        .map_err(InternalError::from)?;
    match v_bytes_opt {
        Some (v_bytes) => {
            let bts = v_bytes.to_vec();
            let v = U::from_bytes(Bytes::from(bts))?;
            Ok(GetResultSuccess::Success(key, v))
        },
        None => Ok(GetResultSuccess::NotFound(key))
    }
}

pub fn contains_key<K: Uid + Borrow<K>> (tree: &Tree, key: K) -> Result<bool, InternalError> {
    let key_bytes = key.to_key_bytes()?;
    Ok(tree.contains_key(key_bytes)?)
}

#[derive(Debug, Clone)]
pub enum PageResultSuccess<U: 'static + Send> {
    Success (U)
}

pub type PageResult<T> = std::result::Result<PageResultSuccess<T>, InternalError>;

#[async_trait]
pub trait Pager where Self: Uid {
    async fn get_page_by_prefix<U: Clone + Send + fmt::Debug +'static> 
        (tree: &Tree, buffer_size: usize, key: Option<Self>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, 
            f: Box<dyn Fn((sled::IVec, sled::IVec)) -> PageResult<U> + Send>)
            -> Result<Receiver<PageResult<U>>, InternalError>;
}

#[async_trait]
impl<T: Uid> Pager for T {
    async fn get_page_by_prefix<U: Clone + Send + fmt::Debug +'static> 
        (tree: &Tree, buffer_size: usize, key: Option<Self>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, 
            f: Box<dyn Fn((sled::IVec, sled::IVec)) -> PageResult<U> + Send>)
            -> Result<Receiver<PageResult<U>>, InternalError>
            {
        let (mut tx, rx) = mpsc::channel::<PageResult<U>>(buffer_size);
        let tree1 = tree.clone();

        let uid = unwrap_field(key, "uid")?;
        let uid_bytes = uid.to_key_bytes()?;    
        
        debug!("Received Uid: {:?}", uid.clone());
        tokio::spawn(async move {
            let actual_page_size = page_size.unwrap_or(default_page_size) as usize;
            let to_skip = (page.unwrap_or(0) as usize) * actual_page_size;
            let page_data: Vec<PageResult<U>> = tree1.scan_prefix(uid_bytes).
                skip(to_skip).
                take(actual_page_size).
                map(|next_v| match next_v {
                    Ok(k) => f(k),
                    Err(e) => Err(e.into())
                }).
                collect();

            debug!("Page: {:?}", page_data.clone());

            for pd in page_data {
                let bts = pd.clone();
                match tx.send(pd).await {
                    Ok(()) => debug!("Sending key: {:?}", bts),// (),
                    Err(e) => error!("Value message transfer failed with: {}", e)
                }
            };
        });

        Ok(rx)
    }
}

pub async fn get_page_by_prefix_u64<U: Send + Clone + fmt::Debug + 'static>
    (tree: &Tree, buffer_size: usize, key: KeyU64, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, 
        f: Box<dyn Fn((sled::IVec, sled::IVec)) -> PageResult<U> + Send>)
         -> Result<Receiver<PageResult<U>>, InternalError> {
    Pager::get_page_by_prefix(tree, buffer_size, Some(key), page, page_size, default_page_size, f).await
}

pub async fn get_page_by_prefix_str<U: Send + Clone + fmt::Debug + 'static>
    (tree: &Tree, buffer_size: usize, key: Option<KeyString>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, 
        f: Box<dyn Fn((sled::IVec, sled::IVec)) -> PageResult<U> + Send>)
         -> Result<Receiver<PageResult<U>>, InternalError> {
    Pager::get_page_by_prefix(tree, buffer_size, key, page, page_size, default_page_size, f).await
}

pub async fn get_page_by_prefix_bytes<U: Send + Clone + fmt::Debug + 'static>
    (tree: &Tree, buffer_size: usize, key: Option<KeyVec>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, 
        f: Box<dyn Fn((sled::IVec, sled::IVec)) -> PageResult<U> + Send>)
         -> Result<Receiver<PageResult<U>>, InternalError> {
    Pager::get_page_by_prefix(tree, buffer_size, key, page, page_size, default_page_size, f).await
}


#[derive(Debug)]
pub enum DeleteResultSuccess<K: Uid> {
    Success (K),
    NotFound (K)
}

pub type DeleteResult<K> = std::result::Result<DeleteResultSuccess<K>, InternalError>;

pub fn delete<K: Uid> (tree: &TransactionalTree, key: K) -> DeleteResult<K> {
    let key_bytes = key.to_key_bytes()?;

    let v = tree.remove(key_bytes).map_err(|e| ConflictableTransactionError::<Option<IVec>>::from(e))?;
    match v {
        Some(_) => Ok(DeleteResultSuccess::Success(key)),
        None => Ok(DeleteResultSuccess::NotFound(key))
    }
}

#[derive(Debug)]
pub enum AddResultSuccess<K: Uid> {
    Success (K),
    Exists (K)
}

pub type AddResult<K> = std::result::Result<AddResultSuccess<K>, InternalError>;

pub fn add<K: Uid, V: ProstMessage> (tree: &TransactionalTree, key: K, value: V) -> AddResult<K> {
    let key_bytes = key.to_key_bytes()?;

    let value_bytes = value.to_bytes()?;

        //let contains = tree.contains_key(uid_bytes)?;
    let contains = tree.get(key_bytes.clone()).map_err(|e| ConflictableTransactionError::<Option<IVec>>::from(e))?;
    match contains { 
        Some(_) => Ok(AddResultSuccess::Exists(key)),
        None => {
            let existing = tree.insert(key_bytes.clone(), value_bytes).map_err(|e| ConflictableTransactionError::<Option<IVec>>::from(e))?;
            if existing.is_some() {
                error!("Unexpected override of the value in store: '{}'", key); 
            }
            Ok(AddResultSuccess::Success(key))
        }
    } 
}