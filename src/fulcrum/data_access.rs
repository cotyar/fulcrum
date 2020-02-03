#![warn(dead_code)]
#![warn(unused_imports)]

use core::mem::size_of;
use core::future::Future;
use tokio::sync::{mpsc, mpsc::*};
use std::fmt;
use std::sync::Arc;
use std::thread;
use crossbeam_epoch::{self as epoch, Atomic, Owned};


use bytes::{Bytes, Buf};

extern crate async_trait;
use async_trait::async_trait;

use crate::pb::*;
use internal_error::{*, Cause::*};

use tracing::{debug, error, Level};

use sled::{Tree};

pub trait ProstMessage : ::prost::Message + Default {}
pub trait Uid : fmt::Debug + Send + Sync + Clone + fmt::Display {
    fn to_key_bytes(self: &Self) -> Result<Vec<u8>, InternalError>;
    fn from_key_bytes<B: Buf>(mut msg_bytes: B) -> Result<Self, InternalError>;
}

impl Uid for u64 {
    fn to_key_bytes(self: &Self) -> Result<Vec<u8>, InternalError> {
        Ok(self.to_be_bytes().to_vec())
    }
    fn from_key_bytes<B: Buf>(mut msg_bytes: B) -> Result<u64, InternalError> {
        Ok(msg_bytes.get_u64())
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
    fn from_key_bytes<B: Buf>(mut msg_bytes: B) -> Result<KeyVec, InternalError> {
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
            let bts: Vec<u8> = v_bytes.iter().cloned().collect();
            match U::from_bytes(Bytes::from(bts)) {
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
pub enum PageResult<U: 'static + ProstMessage> {
    Success (U),
    KeyError (InternalError),
    ValueError (InternalError)
}

#[async_trait]
pub trait Pager where Self: Uid {
    async fn get_page_by_prefix<U: ProstMessage + Clone + 'static, TErr> 
        (tree: &Tree, buffer_size: usize, key: Option<Self>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, 
            f: Box<dyn for<'r> Fn(&'r (sled::IVec, sled::IVec)) -> U + Send>)
            -> Result<Receiver<PageResult<U>>, InternalError>;
}

#[async_trait]
impl<T: Uid> Pager for T {
    async fn get_page_by_prefix<U: ProstMessage + Clone + 'static, TErr> 
        (tree: &Tree, buffer_size: usize, key: Option<Self>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, 
            f: Box<dyn for<'r> Fn(&'r (sled::IVec, sled::IVec)) -> U + Send>)
            -> Result<Receiver<PageResult<U>>, InternalError>
            {
        let (mut tx, rx) = mpsc::channel::<PageResult<U>>(100);
        let tree1 = tree.clone();
        
        match process_uid(key, |_, uid_bytes| Ok(uid_bytes.clone())) {
            Ok((_uid, uid_bytes)) => { 
                tokio::spawn(async move {
                    let mut page_data: Vec<PageResult<U>> = Vec::with_capacity(page_size.unwrap_or(default_page_size) as usize);
                    {
                        let mut iter = tree1.scan_prefix(uid_bytes);

                        for _ in 0..(page_size.unwrap_or(default_page_size)) {
                            let next_v = &iter.next();
                            match next_v {
                                Some(Ok(k)) => {
                                    let v = f(k);
                                    page_data.push(PageResult::Success(v));
                                },
                                Some(Err(e)) => {
                                    page_data.push(PageResult::KeyError(to_internal_error(e.clone())));
                                    break;
                                },
                                None => break
                            }
                        }
                    }

                    for pd in page_data {
                        match tx.send(pd).await {
                            Ok(()) => (),
                            Err(e) => error!("Value message transfer failed with: {}", e)
                        }
                    };
                });

                Ok(rx)
            },
            Err(e) => Err(e)
        }
    }
}

pub async fn get_page_by_prefix_u64<U: ProstMessage + Clone + 'static, TErr, F>
    (tree: Tree, buffer_size: usize, key: u64, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, 
        f: Box<dyn for<'r> Fn(&'r (sled::IVec, sled::IVec)) -> U + Send>)
         -> Result<Receiver<PageResult<U>>, InternalError> {
    Pager::get_page_by_prefix::<U, TErr>(&tree, buffer_size, Some(key), page, page_size, default_page_size, f).await
}

pub async fn get_page_by_prefix_str<U: ProstMessage + Clone + 'static, TErr, F>
    (tree: Tree, buffer_size: usize, key: Option<String>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, 
        f: Box<dyn for<'r> Fn(&'r (sled::IVec, sled::IVec)) -> U + Send>)
         -> Result<Receiver<PageResult<U>>, InternalError> {
    Pager::get_page_by_prefix::<U, TErr>(&tree, buffer_size, key, page, page_size, default_page_size, f).await
}

pub async fn get_page_by_prefix_bytes<U: ProstMessage + Clone + 'static, TErr, F>
    (tree: Tree, buffer_size: usize, key: Option<KeyVec>, page: Option<u32>, page_size: Option<u32>, default_page_size: u32, 
        f: Box<dyn for<'r> Fn(&'r (sled::IVec, sled::IVec)) -> U + Send>)
         -> Result<Receiver<PageResult<U>>, InternalError> {
    Pager::get_page_by_prefix::<U, TErr>(&tree, buffer_size, key, page, page_size, default_page_size, f).await
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