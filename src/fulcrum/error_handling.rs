
use std::*;
use crate::pb::*;
use internal_error::{Cause::*};

// fn format_internal_error(e: InternalError) -> String {
//     match e.cause {
//         None => String::from("Undefined error"),
//         Some(MissingRequiredArgument(s)) => format! ("MissingRequiredArgument '{}'", s), 
//         Some(StorageValueEncodingError(e)) => format! ("StorageValueEncodingError. Required: '{}', remaining: '{}'", e.required, e.remaining),
//         Some(StorageValueDecodingError(e)) => format! ("StorageValueDecodingError '{}', for field: ", e.description), // TODO: Add stack printing
//         Some(StorageError(s)) => format! ("StorageError '{}'", s),
//         Some(TransactionAborted(s)) => format! ("TransactionAborted '{}'", s),
//         Some(KeyError(s)) => format! ("KeyError '{}'", s),
//         Some(ValueError(s)) => format! ("ValueError '{}'", s),
//     }
// } 

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //let msg = format_internal_error(*self);
        write!(f, "An Error Occurred: '{:?}'", self) // user-facing output
    }
}

impl error::Error for InternalError {
    // fn source(&self) -> Option<&(Error + 'static)> {

    // }
}

impl From<::sled::Error> for InternalError {
    fn from(error: ::sled::Error) -> Self {
        InternalError { cause: Some(StorageError(error.to_string())) }
    }
}

impl<T: fmt::Debug> From<::sled::ConflictableTransactionError<T>> for InternalError {
    fn from(e: ::sled::ConflictableTransactionError<T>) -> Self {
        InternalError { cause: Some(TransactionAborted(format!("{:?}", e))) }
    }
}

impl<T: fmt::Debug> From<::sled::TransactionError<T>> for InternalError {
    fn from(e: ::sled::TransactionError<T>) -> Self {
        InternalError { cause: Some(TransactionAborted(format!("{:?}", e))) }
    }
}

pub fn unwrap_field<T>(msg: Option<T>, field_name: &str) -> Result<T, InternalError> { 
    msg.ok_or(InternalError { cause: Some(MissingRequiredArgument(field_name.to_string())) })
}

