pub mod v1;
use osmosis_std_derive::CosmwasmExt;
/// Params defines the parameters for the module.
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
#[proto_message(type_url = "/neutron.interchaintxs.Params")]
pub struct Params {
    /// Defines maximum amount of messages to be passed in MsgSubmitTx
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub msg_submit_tx_max_messages: u64,
}
/// GenesisState defines the interchaintxs module's genesis state.
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
#[proto_message(type_url = "/neutron.interchaintxs.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/neutron.interchaintxs.QueryParamsRequest")]
#[proto_query(
    path = "/neutron.interchaintxs.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/neutron.interchaintxs.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
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
#[proto_message(type_url = "/neutron.interchaintxs.QueryInterchainAccountAddressRequest")]
#[proto_query(
    path = "/neutron.interchaintxs.Query/InterchainAccountAddress",
    response_type = QueryInterchainAccountAddressResponse
)]
pub struct QueryInterchainAccountAddressRequest {
    /// owner_address is the owner of the interchain account on the controller
    /// chain
    #[prost(string, tag = "1")]
    pub owner_address: ::prost::alloc::string::String,
    /// interchain_account_id is an identifier of your interchain account from
    /// which you want to execute msgs
    #[prost(string, tag = "2")]
    #[serde(alias = "interchain_accountID")]
    pub interchain_account_id: ::prost::alloc::string::String,
    /// connection_id is an IBC connection identifier between Neutron and remote
    /// chain
    #[prost(string, tag = "3")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Query response for an interchain account address
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
#[proto_message(type_url = "/neutron.interchaintxs.QueryInterchainAccountAddressResponse")]
pub struct QueryInterchainAccountAddressResponse {
    /// The corresponding interchain account address on the host chain
    #[prost(string, tag = "1")]
    pub interchain_account_address: ::prost::alloc::string::String,
}
pub struct InterchaintxsQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> InterchaintxsQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn interchain_account_address(
        &self,
        owner_address: ::prost::alloc::string::String,
        interchain_account_id: ::prost::alloc::string::String,
        connection_id: ::prost::alloc::string::String,
    ) -> Result<QueryInterchainAccountAddressResponse, cosmwasm_std::StdError> {
        QueryInterchainAccountAddressRequest {
            owner_address,
            interchain_account_id,
            connection_id,
        }
        .query(self.querier)
    }
}
