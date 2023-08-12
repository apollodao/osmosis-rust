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
#[proto_message(type_url = "/neutron.contractmanager.Params")]
pub struct Params {}
/// Failure message contains information about ACK failures and can be used to
/// replay ACK in case of requirement.
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
#[proto_message(type_url = "/neutron.contractmanager.Failure")]
pub struct Failure {
    /// ChannelId
    #[prost(string, tag = "1")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// Address of the failed contract
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// id of the failure under specific address
    #[prost(uint64, tag = "3")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// ACK id to restore
    #[prost(uint64, tag = "4")]
    #[serde(alias = "ackID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ack_id: u64,
    /// Acknowledgement type
    #[prost(string, tag = "5")]
    pub ack_type: ::prost::alloc::string::String,
}
/// GenesisState defines the contractmanager module's genesis state.
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
#[proto_message(type_url = "/neutron.contractmanager.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// List of the contract failures
    #[prost(message, repeated, tag = "2")]
    pub failures_list: ::prost::alloc::vec::Vec<Failure>,
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
#[proto_message(type_url = "/neutron.contractmanager.QueryParamsRequest")]
#[proto_query(
    path = "/neutron.contractmanager.Query/Params",
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
#[proto_message(type_url = "/neutron.contractmanager.QueryParamsResponse")]
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
#[proto_message(type_url = "/neutron.contractmanager.QueryFailuresRequest")]
#[proto_query(
    path = "/neutron.contractmanager.Query/AddressFailures",
    response_type = QueryFailuresResponse
)]
pub struct QueryFailuresRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
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
#[proto_message(type_url = "/neutron.contractmanager.QueryFailuresResponse")]
pub struct QueryFailuresResponse {
    #[prost(message, repeated, tag = "1")]
    pub failures: ::prost::alloc::vec::Vec<Failure>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
pub struct ContractmanagerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ContractmanagerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn address_failures(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryFailuresResponse, cosmwasm_std::StdError> {
        QueryFailuresRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn failures(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryFailuresResponse, cosmwasm_std::StdError> {
        QueryFailuresRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
}
