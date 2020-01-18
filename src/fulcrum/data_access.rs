use std::fmt;

use prost::Message;
use bytes::{Buf, IntoBuf};


use crate::pb::*;
use crate::pb::cdn_control_server::*;
use crate::pb::cdn_query_server::*;
use internal_error::{*, Cause::*};

use tracing::{debug, error, Level};

use sled::Db;

impl fmt::Display for CdnUid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CdnUid: {}", self.message)
    }
}

impl Eq for CdnUid {} 

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

pub trait ProstMessageExt<T: ::prost::Message + Default> {
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

pub fn unwrap_field<T: ::prost::Message + Default>(msg: Option<T>, field_name: &str) -> Result<T, InternalError> { 
    msg.ok_or(InternalError { cause: Some(MissingRequiredArgument(field_name.to_string())) })
}

pub fn process_uid<T> (r_uid: Option<CdnUid>, f: impl FnOnce(&CdnUid, &Vec<u8>) -> Result<T, ::sled::Error>) -> Result<(CdnUid, T), InternalError> {
    let uid = unwrap_field(r_uid, "uid")?;
    let uid_bytes = uid.to_bytes()?;

    let old_value = f(&uid, &uid_bytes)
        .map_err(|e| InternalError { cause: Some(StorageError(e.to_string())) })?;
    Ok((uid, old_value))
}

pub enum GetResult {
    Success (CdnUid, CdnValue),
    NotFound (CdnUid),
    Error (InternalError)
}

pub fn get (db: &Db, key: Option<CdnUid>) -> GetResult {
    match process_uid(key, |_, uid_bytes| db.get(uid_bytes)) {
        Ok((uid, Some(v_bytes))) => {
            match CdnValue::from_bytes(v_bytes.into_buf()) {
                Ok(v) => GetResult::Success(uid, v),
                Err(e) => GetResult::Error(e),
            }
        },
        Ok((uid, None)) => GetResult::NotFound(uid),
        Err(e) => GetResult::Error(e)
    }
}

pub fn contains_key (db: &Db, key: Option<CdnUid>) -> Result<bool, InternalError> {
    process_uid(key, |_, uid_bytes| db.contains_key(uid_bytes)).map(|(_, v)| v)
}

pub enum DeleteResult {
    Success (CdnUid),
    NotFound (CdnUid),
    Error (InternalError)
}

pub fn delete (db: &Db, key: Option<CdnUid>) -> DeleteResult {
    match process_uid(key, |_, uid_bytes| db.remove(uid_bytes)) {
        Ok((uid, Some(_))) => DeleteResult::Success(uid),
        Ok((uid, None)) => DeleteResult::NotFound(uid),
        Err(e) => DeleteResult::Error(e)
    }
}

pub enum AddResult {
    Success (CdnUid),
    Exists (CdnUid),
    Error (InternalError)
}

pub fn add (db: &Db, key: Option<CdnUid>, value: Option<CdnValue>) -> AddResult {
    let res = || -> Result<AddResult, InternalError> {
        let val = unwrap_field(value, "value")?;
        let value_bytes = val.to_bytes()?;

        let check_and_insert = |uid: &CdnUid, uid_bytes: &Vec<u8>| -> Result<_, ::sled::Error> {
            let contains = db.contains_key(uid_bytes)?;
            if contains { 
                Ok(AddResult::Exists(uid.clone()))
            }
            else {
                let existing = db.insert(uid_bytes, value_bytes)?; 
                if existing.is_some() {
                    error!("Unexpected override of the value in store: '{}'", uid); 
                }
                Ok(AddResult::Success(uid.clone()))
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