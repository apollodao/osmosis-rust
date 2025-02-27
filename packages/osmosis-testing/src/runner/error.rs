use cosmrs::rpc::error::Error as TendermintRpcError;
use cosmrs::tendermint::Error as TendermintError;
use cosmrs::ErrorReport;
use cosmwasm_std::StdError;
use std::str::Utf8Error;
use thiserror::Error;
use tonic::transport::Error as TonicError;
use tonic::Status;

#[derive(Error, Debug)]
pub enum RunnerError {
    #[error("unable to encode request")]
    EncodeError(#[from] EncodeError),

    #[error("unable to decode response")]
    DecodeError(#[from] DecodeError),

    #[error("Can't decode invalid protobuf: {0}")]
    ProtoDecodeError(#[from] prost::DecodeError),

    #[error("Can't encode invalid protobuf: {0}")]
    ProtoEncodeError(#[from] prost::EncodeError),

    #[error("query error: {}", .msg)]
    QueryError { msg: String },

    #[error("execute error: {}", .msg)]
    ExecuteError { msg: String },

    #[error("{0}")]
    StdError(#[from] StdError),

    #[error("{0}")]
    Tendermint(#[from] TendermintError),

    #[error("{0}")]
    TendermintRpc(#[from] TendermintRpcError),

    #[error("{0}")]
    ErrorReport(#[from] ErrorReport),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    TonicStatus(#[from] Status),

    #[error("{0}")]
    Tonic(#[from] TonicError),

    #[error("{0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

/// We can't derive PartialEq because TendermintRpcError doesn't implement it.
impl PartialEq for RunnerError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RunnerError::EncodeError(a), RunnerError::EncodeError(b)) => a == b,
            (RunnerError::DecodeError(a), RunnerError::DecodeError(b)) => a == b,
            (RunnerError::QueryError { msg: a }, RunnerError::QueryError { msg: b }) => a == b,
            (RunnerError::ExecuteError { msg: a }, RunnerError::ExecuteError { msg: b }) => a == b,
            (RunnerError::StdError(a), RunnerError::StdError(b)) => a.to_string() == b.to_string(),
            (RunnerError::TendermintRpc(a), RunnerError::TendermintRpc(b)) => a.0 == b.0,
            (RunnerError::Io(a), RunnerError::Io(b)) => a.to_string() == b.to_string(),
            (RunnerError::Tonic(a), RunnerError::Tonic(b)) => a.to_string() == b.to_string(),
            _ => false,
        }
    }
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("invalid utf8 bytes")]
    Utf8Error(#[from] Utf8Error),

    #[error("invalid protobuf")]
    ProtoDecodeError(#[from] prost::DecodeError),

    #[error("invalid json")]
    JsonDecodeError(#[from] serde_json::Error),

    #[error("invalid base64")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("invalid signing key")]
    SigningKeyDecodeError { msg: String },
}

impl PartialEq for DecodeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DecodeError::Utf8Error(a), DecodeError::Utf8Error(b)) => a == b,
            (DecodeError::ProtoDecodeError(a), DecodeError::ProtoDecodeError(b)) => a == b,
            (DecodeError::JsonDecodeError(a), DecodeError::JsonDecodeError(b)) => {
                a.to_string() == b.to_string()
            }
            (DecodeError::Base64DecodeError(a), DecodeError::Base64DecodeError(b)) => a == b,
            (
                DecodeError::SigningKeyDecodeError { msg: a },
                DecodeError::SigningKeyDecodeError { msg: b },
            ) => a == b,
            _ => false,
        }
    }
}

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("invalid protobuf")]
    ProtoEncodeError(#[from] prost::EncodeError),

    #[error("unable to encode json")]
    JsonEncodeError(#[from] serde_json::Error),
}

impl PartialEq for EncodeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EncodeError::ProtoEncodeError(a), EncodeError::ProtoEncodeError(b)) => a == b,
            (EncodeError::JsonEncodeError(a), EncodeError::JsonEncodeError(b)) => {
                a.to_string() == b.to_string()
            }
            _ => false,
        }
    }
}
