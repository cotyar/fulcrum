use core::mem::size_of;
use core::future::Future;
use tokio::sync::{mpsc, mpsc::*};
use std::fmt;

use bytes::{Buf, IntoBuf};


use crate::pb::*;
use internal_error::{*, Cause::*};

use tracing::{debug, error, Level};

use sled::{Tree};

pub trait ProstMessage : ::prost::Message + Default {}
pub trait Uid : Clone + fmt::Display {
    fn to_key_bytes(self: &Self) -> Result<Vec<u8>, InternalError>;
    fn from_key_bytes<B: Buf>(msg_bytes: B) -> Result<Self, InternalError>;
}

impl Uid for u64 {
    fn to_key_bytes(self: &Self) -> Result<Vec<u8>, InternalError> {
        Ok(self.to_be_bytes().to_vec())
    }
    fn from_key_bytes<B: Buf>(msg_bytes: B) -> Result<u64, InternalError> {
        Ok(msg_bytes.take(size_of::<u64>()).get_u64_be())
    }
}

impl Uid for String {
    fn to_key_bytes(self: &Self) -> Result<Vec<u8>, InternalError> {
        Ok(self.bytes().collect())
    }
    fn from_key_bytes<B: Buf>(msg_bytes: B) -> Result<Self, InternalError> {
        String::from_utf8(msg_bytes.bytes().iter().cloned().collect()).
            map_err(|e| InternalError { cause: Some(StorageValueDecodingError(DecodeError{ description: e.to_string(), stack: Vec::new() }))})
    }
}


#[derive(Debug, Clone)]
pub struct KeyVec(Vec<u8>);

impl Uid for KeyVec {
    fn to_key_bytes(self: &Self) -> Result<Vec<u8>, InternalError> {
        Ok(self.0.clone())
    }
    fn from_key_bytes<B: Buf>(msg_bytes: B) -> Result<KeyVec, InternalError> {
        Ok(KeyVec(msg_bytes.iter().collect()))
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
impl ProstMessage for Entry {}

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

fn to_internal_error(e: sled::Error) -> InternalError {
    InternalError { cause: Some(StorageError(e.to_string())) }
}

pub fn unwrap_field<T>(msg: Option<T>, field_name: &str) -> Result<T, InternalError> { 
    msg.ok_or(InternalError { cause: Some(MissingRequiredArgument(field_name.to_string())) })
}

pub fn process_uid<T: Uid, U> (r_uid: Option<T>, f: impl FnOnce(&T, &Vec<u8>) -> Result<U, ::sled::Error>) -> Result<(T, U), InternalError> {
    let uid = unwrap_field(r_uid, "uid")?;
    let uid_bytes = uid.to_key_bytes()?;

    let old_value = f(&uid, &uid_bytes).map_err(|e| to_internal_error (e))?;
    Ok((uid, old_value))
}

#[derive(Debug)]
pub enum GetResult<T: Uid, U: ProstMessage> {
    Success (T, U),
    NotFound (T),
    Error (InternalError)
}

pub fn get<T: Uid, U: ProstMessage> (tree: &Tree, key: Option<T>) -> GetResult<T, U> {
    match process_uid(key, |_, uid_bytes| tree.get(uid_bytes)) {
        Ok((uid, Some(v_bytes))) => {
            match U::from_bytes(v_bytes.into_buf()) {
                Ok(v) => GetResult::Success(uid, v),
                Err(e) => GetResult::Error(e),
            }
        },
        Ok((uid, None)) => GetResult::NotFound(uid),
        Err(e) => GetResult::Error(e)
    }
}

pub fn contains_key<T: Uid + Default> (tree: &Tree, key: Option<T>) -> Result<bool, InternalError> {
    process_uid(key, |_, uid_bytes| tree.contains_key(uid_bytes)).map(|(_, v)| v)
}

#[derive(Debug)]
pub enum PageResult<U: ProstMessage> {
    Success (U),
    KeyError (InternalError),
    ValueError (InternalError)
}
//#[instrument]
async fn send_response_msg<T: std::fmt::Debug> (tx: &mut Sender<T>, msg: T) {
    debug!("StreamValueStream sending: {:?}", &msg);
    match tx.send(msg).await {
        Ok(()) => (),
        Err(e) => error!("Value message transfer failed with: {}", e)
    }
}
pub async fn get_page_by_prefix<T: Uid, U: ProstMessage, TErr, F> 
    (tree: Tree, buffer_size: usize, include_values: bool, key: Option<T>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, f: F)
         -> Result<Receiver<PageResult<U>>, InternalError> 
         where F: Fn(sled::IVec) -> U {

    match process_uid(key, |_, uid_bytes| Ok(uid_bytes.clone())) {
        Ok((uid, uid_bytes)) => { //match iter {
            // Ok((key_iter, value_iter)) => {
                // let value_iter = &iter.values();
                let (mut tx, rx) = mpsc::channel(buffer_size);
                tokio::spawn(async move {
                    let iter = tree.scan_prefix(uid_bytes);
                    let key_iter = &mut iter.keys();
                    // let value_iter = &mut iter.values();    

                    for kv in 0..(page_size.unwrap_or(default_page_size)) {
                        match key_iter.next() {
                            Some(Ok(k)) => (), //send_response_msg(&mut tx, PageResult::Success(f(k))).await,
                            Some(Err(e)) => {
                                // send_response_msg(&mut tx, PageResult::KeyError(to_internal_error(e))).await;
                                break;
                            },
                            None => break
                        }
                    }
                });

                Ok(rx)
            // },
            // Err(e) => InternalError { cause: Some(StorageError(e.to_string())) }
        },
        Err(e) => Err(e)
    }
}

pub async fn get_page_by_prefix_u64<U: ProstMessage, TErr, F>
    (tree: Tree, buffer_size: usize, include_values: bool, key: Option<u64>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, f: F)
         -> Result<Receiver<PageResult<U>>, InternalError> 
         where F: Fn(sled::IVec) -> U {
    get_page_by_prefix::<_, _, TErr, _>(tree, buffer_size, include_values, key, page, page_size, default_page_size, f).await
}

pub async fn get_page_by_prefix_str<U: ProstMessage, TErr, F>
    (tree: Tree, buffer_size: usize, include_values: bool, key: Option<String>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, f: F)
         -> Result<Receiver<PageResult<U>>, InternalError> 
         where F: Fn(sled::IVec) -> U {
    get_page_by_prefix::<_, _, TErr, _>(tree, buffer_size, include_values, key, page, page_size, default_page_size, f).await
}

pub async fn get_page_by_prefix_bytes<U: ProstMessage, TErr, F>
    (tree: Tree, buffer_size: usize, include_values: bool, key: Option<KeyVec>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, f: F)
         -> Result<Receiver<PageResult<U>>, InternalError> 
         where F: Fn(sled::IVec) -> U {
    get_page_by_prefix::<_, _, TErr, _>(tree, buffer_size, include_values, key, page, page_size, default_page_size, f).await
}

#[derive(Debug)]
pub enum DeleteResult<T: Uid> {
    Success (T),
    NotFound (T),
    Error (InternalError)
}

pub fn delete<T: Uid> (tree: &Tree, key: Option<T>) -> DeleteResult<T> {
    match process_uid(key, |_, uid_bytes| tree.remove(uid_bytes)) {
        Ok((uid, Some(_))) => DeleteResult::Success(uid),
        Ok((uid, None)) => DeleteResult::NotFound(uid),
        Err(e) => DeleteResult::Error(e)
    }
}

#[derive(Debug)]
pub enum AddResult<T: Uid> {
    Success (T),
    Exists (T),
    Error (InternalError)
}

pub fn add<T: Uid, U: ProstMessage> (tree: &Tree, key: Option<T>, value: Option<U>) -> AddResult<T> {
    let res = || -> Result<AddResult<T>, InternalError> {
        let val = unwrap_field(value, "value")?;
        let value_bytes = val.to_bytes()?;

        let check_and_insert = |uid: &T, uid_bytes: &Vec<u8>| -> Result<_, ::sled::Error> {
            let contains = tree.contains_key(uid_bytes)?;
            if contains { 
                Ok(AddResult::<T>::Exists(uid.clone()))
            }
            else {
                let existing = tree.insert(uid_bytes, value_bytes)?; 
                if existing.is_some() {
                    error!("Unexpected override of the value in store: '{}'", uid); 
                }
                Ok(AddResult::<T>::Success(uid.clone()))
            } 
        };    
        
        let (_, ret) = process_uid(key, check_and_insert)?;
        Ok(ret)
    };
    
    match res() {
        Ok(resp) => resp,
        Err(e) => AddResult::Error(e)
    }
}