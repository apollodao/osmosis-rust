use osmosis_std_derive::CosmwasmExt;
/// Permissions are the permissions that an account has to trip
/// or reset the circuit breaker.
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
#[proto_message(type_url = "/cosmos.circuit.v1.Permissions")]
pub struct Permissions {
    /// level is the level of permissions granted to this account.
    #[prost(enumeration = "permissions::Level", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub level: i32,
    /// limit_type_urls is used with LEVEL_SOME_MSGS to limit the lists of Msg type
    /// URLs that the account can trip. It is an error to use limit_type_urls with
    /// a level other than LEVEL_SOME_MSGS.
    #[prost(string, repeated, tag = "2")]
    pub limit_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Permissions`.
pub mod permissions {
    use osmosis_std_derive::CosmwasmExt;
    /// Level is the permission level.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
    pub enum Level {
        /// LEVEL_NONE_UNSPECIFIED indicates that the account will have no circuit
        /// breaker permissions.
        NoneUnspecified = 0,
        /// LEVEL_SOME_MSGS indicates that the account will have permission to
        /// trip or reset the circuit breaker for some Msg type URLs. If this level
        /// is chosen, a non-empty list of Msg type URLs must be provided in
        /// limit_type_urls.
        SomeMsgs = 1,
        /// LEVEL_ALL_MSGS indicates that the account can trip or reset the circuit
        /// breaker for Msg's of all type URLs.
        AllMsgs = 2,
        /// LEVEL_SUPER_ADMIN indicates that the account can take all circuit breaker
        /// actions and can grant permissions to other accounts.
        SuperAdmin = 3,
    }
    impl Level {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Level::NoneUnspecified => "LEVEL_NONE_UNSPECIFIED",
                Level::SomeMsgs => "LEVEL_SOME_MSGS",
                Level::AllMsgs => "LEVEL_ALL_MSGS",
                Level::SuperAdmin => "LEVEL_SUPER_ADMIN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LEVEL_NONE_UNSPECIFIED" => Some(Self::NoneUnspecified),
                "LEVEL_SOME_MSGS" => Some(Self::SomeMsgs),
                "LEVEL_ALL_MSGS" => Some(Self::AllMsgs),
                "LEVEL_SUPER_ADMIN" => Some(Self::SuperAdmin),
                _ => None,
            }
        }
    }
}
/// GenesisAccountPermissions is the account permissions for the circuit breaker in genesis
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
#[proto_message(type_url = "/cosmos.circuit.v1.GenesisAccountPermissions")]
pub struct GenesisAccountPermissions {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub permissions: ::core::option::Option<Permissions>,
}
/// GenesisState is the state that must be provided at genesis.
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
#[proto_message(type_url = "/cosmos.circuit.v1.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub account_permissions: ::prost::alloc::vec::Vec<GenesisAccountPermissions>,
    #[prost(string, repeated, tag = "2")]
    pub disabled_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryAccountRequest is the request type for the Query/Account RPC method.
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
#[proto_message(type_url = "/cosmos.circuit.v1.QueryAccountRequest")]
#[proto_query(
    path = "/cosmos.circuit.v1.Query/Account",
    response_type = AccountResponse
)]
pub struct QueryAccountRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// AccountResponse is the response type for the Query/Account RPC method.
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
#[proto_message(type_url = "/cosmos.circuit.v1.AccountResponse")]
pub struct AccountResponse {
    #[prost(message, optional, tag = "1")]
    pub permission: ::core::option::Option<Permissions>,
}
/// QueryAccountsRequest is the request type for the Query/Accounts RPC method.
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
#[proto_message(type_url = "/cosmos.circuit.v1.QueryAccountsRequest")]
#[proto_query(
    path = "/cosmos.circuit.v1.Query/Accounts",
    response_type = AccountsResponse
)]
pub struct QueryAccountsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// AccountsResponse is the response type for the Query/Accounts RPC method.
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
#[proto_message(type_url = "/cosmos.circuit.v1.AccountsResponse")]
pub struct AccountsResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<GenesisAccountPermissions>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDisableListRequest is the request type for the Query/DisabledList RPC method.
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
#[proto_message(type_url = "/cosmos.circuit.v1.QueryDisabledListRequest")]
#[proto_query(
    path = "/cosmos.circuit.v1.Query/DisabledList",
    response_type = DisabledListResponse
)]
pub struct QueryDisabledListRequest {}
/// DisabledListResponse is the response type for the Query/DisabledList RPC method.
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
#[proto_message(type_url = "/cosmos.circuit.v1.DisabledListResponse")]
pub struct DisabledListResponse {
    #[prost(string, repeated, tag = "1")]
    pub disabled_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgAuthorizeCircuitBreaker defines the Msg/AuthorizeCircuitBreaker request type.
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
#[proto_message(type_url = "/cosmos.circuit.v1.MsgAuthorizeCircuitBreaker")]
pub struct MsgAuthorizeCircuitBreaker {
    /// granter is the granter of the circuit breaker permissions and must have
    /// LEVEL_SUPER_ADMIN.
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    /// grantee is the account authorized with the provided permissions.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// permissions are the circuit breaker permissions that the grantee receives.
    /// These will overwrite any existing permissions. LEVEL_NONE_UNSPECIFIED can
    /// be specified to revoke all permissions.
    #[prost(message, optional, tag = "3")]
    pub permissions: ::core::option::Option<Permissions>,
}
/// MsgAuthorizeCircuitBreakerResponse defines the Msg/AuthorizeCircuitBreaker response type.
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
#[proto_message(type_url = "/cosmos.circuit.v1.MsgAuthorizeCircuitBreakerResponse")]
pub struct MsgAuthorizeCircuitBreakerResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// MsgTripCircuitBreaker defines the Msg/TripCircuitBreaker request type.
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
#[proto_message(type_url = "/cosmos.circuit.v1.MsgTripCircuitBreaker")]
pub struct MsgTripCircuitBreaker {
    /// authority is the account authorized to trip the circuit breaker.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// msg_type_urls specifies a list of type URLs to immediately stop processing.
    /// IF IT IS LEFT EMPTY, ALL MSG PROCESSING WILL STOP IMMEDIATELY.
    /// This value is validated against the authority's permissions and if the
    /// authority does not have permissions to trip the specified msg type URLs
    /// (or all URLs), the operation will fail.
    #[prost(string, repeated, tag = "2")]
    pub msg_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgTripCircuitBreakerResponse defines the Msg/TripCircuitBreaker response type.
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
#[proto_message(type_url = "/cosmos.circuit.v1.MsgTripCircuitBreakerResponse")]
pub struct MsgTripCircuitBreakerResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// MsgResetCircuitBreaker defines the Msg/ResetCircuitBreaker request type.
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
#[proto_message(type_url = "/cosmos.circuit.v1.MsgResetCircuitBreaker")]
pub struct MsgResetCircuitBreaker {
    /// authority is the account authorized to trip or reset the circuit breaker.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// msg_type_urls specifies a list of Msg type URLs to resume processing. If
    /// it is left empty all Msg processing for type URLs that the account is
    /// authorized to trip will resume.
    #[prost(string, repeated, tag = "3")]
    pub msg_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgResetCircuitBreakerResponse defines the Msg/ResetCircuitBreaker response type.
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
#[proto_message(type_url = "/cosmos.circuit.v1.MsgResetCircuitBreakerResponse")]
pub struct MsgResetCircuitBreakerResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
pub struct CircuitQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> CircuitQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn account(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<AccountResponse, cosmwasm_std::StdError> {
        QueryAccountRequest { address }.query(self.querier)
    }
    pub fn accounts(
        &self,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<AccountsResponse, cosmwasm_std::StdError> {
        QueryAccountsRequest { pagination }.query(self.querier)
    }
    pub fn disabled_list(&self) -> Result<DisabledListResponse, cosmwasm_std::StdError> {
        QueryDisabledListRequest {}.query(self.querier)
    }
}
