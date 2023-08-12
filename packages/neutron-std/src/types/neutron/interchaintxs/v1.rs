use osmosis_std_derive::CosmwasmExt;
/// MsgRegisterInterchainAccount is used to register an account on a remote zone.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchaintxs.v1.MsgRegisterInterchainAccount")]
pub struct MsgRegisterInterchainAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(alias = "interchain_accountID")]
    pub interchain_account_id: ::prost::alloc::string::String,
}
/// MsgRegisterInterchainAccountResponse is the response type for
/// MsgRegisterInterchainAccount.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchaintxs.v1.MsgRegisterInterchainAccountResponse")]
pub struct MsgRegisterInterchainAccountResponse {}
/// MsgSubmitTx defines the payload for Msg/SubmitTx
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchaintxs.v1.MsgSubmitTx")]
pub struct MsgSubmitTx {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    /// interchain_account_id is supposed to be the unique identifier, e.g.,
    /// lido/kava. This allows contracts to have more than one interchain accounts
    /// on remote zone This identifier will be a part of the portID that we'll
    /// claim our capability for.
    #[prost(string, tag = "2")]
    #[serde(alias = "interchain_accountID")]
    pub interchain_account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub msgs: ::prost::alloc::vec::Vec<crate::shim::Any>,
    #[prost(string, tag = "5")]
    pub memo: ::prost::alloc::string::String,
    /// timeout in seconds after which the packet times out
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timeout: u64,
    #[prost(message, optional, tag = "7")]
    pub fee: ::core::option::Option<super::super::feerefunder::Fee>,
}
/// MsgSubmitTxResponse defines the response for Msg/SubmitTx
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchaintxs.v1.MsgSubmitTxResponse")]
pub struct MsgSubmitTxResponse {
    /// channel's sequence_id for outgoing ibc packet. Unique per a channel.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "sequenceID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence_id: u64,
    /// channel src channel on neutron side trasaction was submitted from
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
}
