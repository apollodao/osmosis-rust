use cosmwasm_schema::{cw_serde, QueryResponses};
pub use osmosis_std::types::osmosis::epochs::v1beta1::QueryEpochsInfoResponse;
pub use osmosis_std::types::osmosis::gamm::v1beta1::{
    QueryNumPoolsResponse, QueryPoolParamsResponse, QueryPoolResponse,
};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub debug: bool,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryNumPoolsResponse)]
    QueryNumPools {},

    #[returns(QueryEpochsInfoResponse)]
    QueryEpochsInfo {},

    #[returns(QueryPoolResponse)]
    QueryPool { pool_id: u64 },

    #[returns(QueryPoolParamsResponse)]
    QueryPoolParams { pool_id: u64 },
}
