use std::ffi::CString;

use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::proto::cosmos::bank::v1beta1::{
    QueryBalanceRequest, QueryBalanceResponse, QuerySupplyOfRequest,
};
use cosmrs::proto::cosmwasm::wasm::v1::{
    QuerySmartContractStateRequest, QuerySmartContractStateResponse,
};
use cosmrs::proto::tendermint::abci::{RequestDeliverTx, ResponseDeliverTx};
use cosmrs::tx;
use cosmrs::tx::{Fee, SignerInfo};
use cosmwasm_std::{
    from_binary, to_binary, BalanceResponse, BankQuery, Binary, Coin, ContractResult, Empty,
    QuerierResult, QueryRequest, SystemResult, WasmQuery,
};
use osmosis_std::types::cosmos::bank::v1beta1::QuerySupplyOfResponse;
use osmosis_std::types::osmosis::gamm::v1beta1::{
    QueryCalcJoinPoolNoSwapSharesResponse, QueryCalcJoinPoolSharesResponse,
};
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::account::{Account, FeeSetting, SigningAccount};
use crate::bindings::{
    AccountNumber, AccountSequence, BeginBlock, EndBlock, Execute, IncreaseTime, InitAccount,
    InitTestEnv, Query, Simulate, WhitelistAddressForForceUnlock,
};
use crate::redefine_as_go_string;
use crate::runner::error::{DecodeError, EncodeError, RunnerError};
use crate::runner::result::RawResult;
use crate::runner::result::{RunnerExecuteResult, RunnerResult};
use crate::runner::Runner;
use crate::utils::{osmosis_proto_coin_to_coin, proto_coin_to_coin};

const FEE_DENOM: &str = "uosmo";
const CHAIN_ID: &str = "osmosis-1";
const DEFAULT_GAS_ADJUSTMENT: f64 = 1.2;

#[derive(Debug, PartialEq, Eq)]
pub struct OsmosisTestApp {
    id: u64,
}

impl Default for OsmosisTestApp {
    fn default() -> Self {
        OsmosisTestApp::new()
    }
}

impl OsmosisTestApp {
    pub fn new() -> Self {
        Self {
            id: unsafe { InitTestEnv() },
        }
    }

    /// Increase the time of the blockchain by the given number of seconds.
    pub fn increase_time(&self, seconds: u64) {
        unsafe {
            IncreaseTime(self.id, seconds);
        }
    }

    pub fn whitelist_address_for_force_unlock(&self, address: &str) {
        redefine_as_go_string!(address);
        unsafe {
            WhitelistAddressForForceUnlock(self.id, address);
        }
    }

    /// Initialize account with initial balance of any coins.
    /// This function mints new coins and send to newly created account
    pub fn init_account(&self, coins: &[Coin]) -> RunnerResult<SigningAccount> {
        let mut coins = coins.to_vec();

        // invalid coins if denom are unsorted
        coins.sort_by(|a, b| a.denom.cmp(&b.denom));

        let coins_json = serde_json::to_string(&coins).map_err(EncodeError::JsonEncodeError)?;
        redefine_as_go_string!(coins_json);

        let base64_priv = unsafe {
            BeginBlock(self.id);
            let addr = InitAccount(self.id, coins_json);
            EndBlock(self.id);
            CString::from_raw(addr)
        }
        .to_str()
        .map_err(DecodeError::Utf8Error)?
        .to_string();

        let secp256k1_priv = base64::decode(base64_priv).map_err(DecodeError::Base64DecodeError)?;
        let signging_key = SigningKey::from_bytes(&secp256k1_priv).map_err(|e| {
            let msg = e.to_string();
            DecodeError::SigningKeyDecodeError { msg }
        })?;

        Ok(SigningAccount::new(
            "osmo".to_string(),
            signging_key,
            FeeSetting::Auto {
                gas_price: Coin::new(0, FEE_DENOM.to_string()),
                gas_adjustment: DEFAULT_GAS_ADJUSTMENT,
            },
        ))
    }
    /// Convinience function to create multiple accounts with the same
    /// Initial coins balance
    pub fn init_accounts(&self, coins: &[Coin], count: u64) -> RunnerResult<Vec<SigningAccount>> {
        (0..count)
            .into_iter()
            .map(|_| self.init_account(coins))
            .collect()
    }

    fn create_signed_tx<I>(
        &self,
        msgs: I,
        signer: &SigningAccount,
        fee: Fee,
    ) -> RunnerResult<Vec<u8>>
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        let tx_body = tx::Body::new(msgs, "", 0u32);
        let addr = signer.address();
        redefine_as_go_string!(addr);

        let seq = unsafe { AccountSequence(self.id, addr) };

        let account_number = unsafe { AccountNumber(self.id, addr) };
        let signer_info = SignerInfo::single_direct(Some(signer.public_key()), seq);
        let auth_info = signer_info.auth_info(fee);
        let sign_doc = tx::SignDoc::new(
            &tx_body,
            &auth_info,
            &(CHAIN_ID
                .parse()
                .expect("parse const str of chain id should never fail")),
            account_number,
        )
        .map_err(|e| match e.downcast::<prost::EncodeError>() {
            Ok(encode_err) => EncodeError::ProtoEncodeError(encode_err),
            Err(e) => panic!("expect `prost::EncodeError` but got {:?}", e),
        })?;

        let tx_raw = sign_doc.sign(signer.signing_key()).unwrap();

        tx_raw
            .to_bytes()
            .map_err(|e| match e.downcast::<prost::EncodeError>() {
                Ok(encode_err) => EncodeError::ProtoEncodeError(encode_err),
                Err(e) => panic!("expect `prost::EncodeError` but got {:?}", e),
            })
            .map_err(RunnerError::EncodeError)
    }

    pub fn simulate_tx<I>(
        &self,
        msgs: I,
        signer: &SigningAccount,
    ) -> RunnerResult<cosmrs::proto::cosmos::base::abci::v1beta1::GasInfo>
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        let zero_fee = Fee::from_amount_and_gas(
            cosmrs::Coin {
                denom: FEE_DENOM.parse().unwrap(),
                amount: 0u8.into(),
            },
            0u64,
        );

        let tx = self.create_signed_tx(msgs, signer, zero_fee)?;
        let base64_tx_bytes = base64::encode(&tx);
        redefine_as_go_string!(base64_tx_bytes);

        unsafe {
            let res = Simulate(self.id, base64_tx_bytes);
            let res = RawResult::from_non_null_ptr(res).into_result()?;

            cosmrs::proto::cosmos::base::abci::v1beta1::GasInfo::decode(res.as_slice())
                .map_err(DecodeError::ProtoDecodeError)
                .map_err(RunnerError::DecodeError)
        }
    }
    fn estimate_fee<I>(&self, msgs: I, signer: &SigningAccount) -> RunnerResult<Fee>
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        match &signer.fee_setting() {
            FeeSetting::Auto {
                gas_price,
                gas_adjustment,
            } => {
                let gas_info = self.simulate_tx(msgs, signer)?;
                let gas_limit = ((gas_info.gas_used as f64) * (gas_adjustment)).ceil() as u64;

                let amount = cosmrs::Coin {
                    denom: FEE_DENOM.parse().unwrap(),
                    amount: (((gas_limit as f64) * (gas_price.amount.u128() as f64)).ceil() as u64)
                        .into(),
                };

                Ok(Fee::from_amount_and_gas(amount, gas_limit))
            }
            FeeSetting::Custom { .. } => {
                panic!("estimate fee is a private function and should never be called when fee_setting is Custom");
            }
        }
    }
}

impl cosmwasm_std::Querier for OsmosisTestApp {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let x = match from_binary::<QueryRequest<Empty>>(&bin_request.into()).unwrap() {
            QueryRequest::Wasm(wasm_query) => match wasm_query {
                WasmQuery::Smart { contract_addr, msg } => self
                    .query::<_, QuerySmartContractStateResponse>(
                        "/cosmwasm.wasm.v1.Query/SmartContractState",
                        &QuerySmartContractStateRequest {
                            address: contract_addr.to_owned(),
                            query_data: msg.into(),
                        },
                    )
                    .unwrap()
                    .data
                    .into(),
                _ => todo!("unsupported WasmQuery variant"),
            },
            QueryRequest::Bank(bank_query) => match bank_query {
                BankQuery::Balance { address, denom } => {
                    let balance = self
                        .query::<_, QueryBalanceResponse>(
                            "/cosmos.bank.v1beta1.Query/Balance",
                            &QueryBalanceRequest {
                                address: address.to_owned(),
                                denom: denom.to_owned(),
                            },
                        )
                        .unwrap()
                        .balance
                        .unwrap();
                    to_binary(&BalanceResponse {
                        amount: proto_coin_to_coin(&balance),
                    })
                    .unwrap()
                }
                BankQuery::Supply { denom } => {
                    let supply = self
                        .query::<_, QuerySupplyOfResponse>(
                            "/cosmos.bank.v1beta1.Query/SupplyOf",
                            &QuerySupplyOfRequest {
                                denom: denom.to_owned(),
                            },
                        )
                        .unwrap()
                        .amount
                        .unwrap();

                    // We must copy this struct because the original is non-exhaustive
                    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
                    #[serde(rename_all = "snake_case")]
                    pub struct SupplyResponse {
                        pub amount: Coin,
                    }
                    to_binary(&SupplyResponse {
                        amount: osmosis_proto_coin_to_coin(&supply),
                    })
                    .unwrap()
                }
                _ => todo!("unsupported BankQuery variant"),
            },
            QueryRequest::Stargate { path, data } => {
                let proto_binary: Binary = self.query_raw(&path, data.into()).unwrap().into();
                match path.as_str() {
                    "/cosmos.bank.v1beta1.Query/SupplyOf" => {
                        let proto: QuerySupplyOfResponse = proto_binary.try_into().unwrap();
                        to_binary(&proto).unwrap()
                    }
                    "/osmosis.gamm.v1beta1.Query/CalcJoinPoolNoSwapShares" => {
                        let proto: QueryCalcJoinPoolNoSwapSharesResponse =
                            proto_binary.try_into().unwrap();
                        to_binary(&proto).unwrap()
                    }
                    "/osmosis.gamm.v1beta1.Query/CalcJoinPoolShares" => {
                        let proto: QueryCalcJoinPoolSharesResponse =
                            proto_binary.try_into().unwrap();
                        to_binary(&proto).unwrap()
                    }
                    _ => todo!("Unsupported Stargate query. path = {}", path),
                }
            }
            _ => todo!("unsupported QueryRequest variant"),
        };

        SystemResult::Ok(ContractResult::Ok(x))
    }
}

impl<'a> Runner<'a> for OsmosisTestApp {
    fn execute_multiple<M, R>(
        &self,
        msgs: &[(M, &str)],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        M: ::prost::Message,
        R: ::prost::Message + Default,
    {
        let msgs = msgs
            .iter()
            .map(|(msg, type_url)| {
                let mut buf = Vec::new();
                M::encode(msg, &mut buf).map_err(EncodeError::ProtoEncodeError)?;

                Ok(cosmrs::Any {
                    type_url: type_url.to_string(),
                    value: buf,
                })
            })
            .collect::<Result<Vec<cosmrs::Any>, RunnerError>>()?;

        self.execute_multiple_raw(msgs, signer)
    }

    fn execute_multiple_raw<R>(
        &self,
        msgs: Vec<cosmrs::Any>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        R: ::prost::Message + Default,
    {
        unsafe { BeginBlock(self.id) };

        let execute_multiple_raw_inner = || -> RunnerExecuteResult<R> {
            let fee = match &signer.fee_setting() {
                FeeSetting::Auto { .. } => self.estimate_fee(msgs.clone(), signer)?,
                FeeSetting::Custom { amount, gas_limit } => Fee::from_amount_and_gas(
                    cosmrs::Coin {
                        denom: amount.denom.parse().unwrap(),
                        amount: amount.amount.to_string().parse().unwrap(),
                    },
                    *gas_limit,
                ),
            };

            let tx = self.create_signed_tx(msgs, signer, fee)?;

            let mut buf = Vec::new();
            RequestDeliverTx::encode(&RequestDeliverTx { tx }, &mut buf)
                .map_err(EncodeError::ProtoEncodeError)?;

            let base64_req = base64::encode(buf);
            redefine_as_go_string!(base64_req);
            let res = unsafe {
                let res = Execute(self.id, base64_req);
                let res = RawResult::from_non_null_ptr(res).into_result()?;

                ResponseDeliverTx::decode(res.as_slice()).map_err(DecodeError::ProtoDecodeError)
            }?
            .try_into();
            res
        };

        // Even if the tx fails we must still call EndBlock
        let res = execute_multiple_raw_inner().map_err(|e| {
            unsafe { EndBlock(self.id) };
            e
        })?;

        unsafe { EndBlock(self.id) };

        Ok(res)
    }

    fn query_raw(&self, path: &str, protobuf: Vec<u8>) -> RunnerResult<Vec<u8>> {
        let base64_query_msg_bytes = base64::encode(protobuf);
        redefine_as_go_string!(path);
        redefine_as_go_string!(base64_query_msg_bytes);

        unsafe {
            let res = Query(self.id, path, base64_query_msg_bytes);
            let res = RawResult::from_non_null_ptr(res).into_result()?;
            Ok(res)
        }
    }

    fn query<Q, R>(&self, path: &str, q: &Q) -> RunnerResult<R>
    where
        Q: ::prost::Message,
        R: ::prost::Message + Default,
    {
        let mut buf = Vec::new();

        Q::encode(q, &mut buf).map_err(EncodeError::ProtoEncodeError)?;

        let res = self.query_raw(path, buf)?;
        R::decode(res.as_slice())
            .map_err(DecodeError::ProtoDecodeError)
            .map_err(RunnerError::DecodeError)
    }
}

#[cfg(test)]
mod tests {
    use std::option::Option::None;

    use cosmrs::proto::cosmos::bank::v1beta1::QueryAllBalancesRequest;
    use cosmwasm_std::{attr, coins, Coin, Empty, QuerierWrapper, Uint128};

    use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, MsgCreateDenomResponse, QueryParamsRequest, QueryParamsResponse,
    };

    use crate::account::{Account, FeeSetting};
    use crate::module::Gamm;
    use crate::module::Module;
    use crate::module::Wasm;
    use crate::runner::app::OsmosisTestApp;
    use crate::runner::*;
    use crate::{Bank, ExecuteResponse};

    #[test]
    fn test_init_accounts() {
        let app = OsmosisTestApp::default();
        let accounts = app
            .init_accounts(&coins(100_000_000_000, "uosmo"), 3)
            .unwrap();

        assert!(accounts.get(0).is_some());
        assert!(accounts.get(1).is_some());
        assert!(accounts.get(2).is_some());
        assert!(accounts.get(3).is_none());
    }

    #[test]
    fn test_execute() {
        let app = OsmosisTestApp::default();

        let acc = app.init_account(&coins(100_000_000_000, "uosmo")).unwrap();
        let addr = acc.address();

        let msg = MsgCreateDenom {
            sender: acc.address(),
            subdenom: "newdenom".to_string(),
        };

        let res: ExecuteResponse<MsgCreateDenomResponse> =
            app.execute(msg, MsgCreateDenom::TYPE_URL, &acc).unwrap();

        let create_denom_attrs = &res
            .events
            .iter()
            .find(|e| e.ty == "create_denom")
            .unwrap()
            .attributes;

        assert_eq!(
            create_denom_attrs,
            &vec![
                attr("creator", &addr),
                attr(
                    "new_token_denom",
                    format!("factory/{}/{}", &addr, "newdenom")
                )
            ]
        );

        // execute on more time to excercise account sequence
        let msg = MsgCreateDenom {
            sender: acc.address(),
            subdenom: "newerdenom".to_string(),
        };

        let res: ExecuteResponse<MsgCreateDenomResponse> =
            app.execute(msg, MsgCreateDenom::TYPE_URL, &acc).unwrap();

        let create_denom_attrs = &res
            .events
            .iter()
            .find(|e| e.ty == "create_denom")
            .unwrap()
            .attributes;

        // TODO: make assertion based on string representation
        assert_eq!(
            create_denom_attrs,
            &vec![
                attr("creator", &addr),
                attr(
                    "new_token_denom",
                    format!("factory/{}/{}", &addr, "newerdenom")
                )
            ]
        );
    }

    #[test]
    fn test_query() {
        let app = OsmosisTestApp::default();

        let denom_creation_fee = app
            .query::<QueryParamsRequest, QueryParamsResponse>(
                "/osmosis.tokenfactory.v1beta1.Query/Params",
                &QueryParamsRequest {},
            )
            .unwrap()
            .params
            .unwrap()
            .denom_creation_fee;

        assert_eq!(denom_creation_fee, [Coin::new(10000000, "uosmo").into()])
    }

    #[test]
    fn test_multiple_as_module() {
        let app = OsmosisTestApp::default();
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let gamm = Gamm::new(&app);

        let pool_liquidity = vec![Coin::new(1_000, "uatom"), Coin::new(1_000, "uosmo")];
        let pool_id = gamm
            .create_basic_pool(&pool_liquidity, &alice)
            .unwrap()
            .data
            .pool_id;

        let pool = gamm.query_pool(pool_id).unwrap();

        assert_eq!(
            pool_liquidity
                .into_iter()
                .map(|c| c.into())
                .collect::<Vec<osmosis_std::types::cosmos::base::v1beta1::Coin>>(),
            pool.pool_assets
                .into_iter()
                .map(|a| a.token.unwrap())
                .collect::<Vec<osmosis_std::types::cosmos::base::v1beta1::Coin>>(),
        );

        let wasm = Wasm::new(&app);
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, &alice)
            .unwrap()
            .data
            .code_id;

        assert_eq!(code_id, 1);
    }

    #[test]
    fn test_wasm_execute_and_query() {
        use cw1_whitelist::msg::*;

        let app = OsmosisTestApp::default();
        let accs = app
            .init_accounts(
                &[
                    Coin::new(1_000_000_000_000, "uatom"),
                    Coin::new(1_000_000_000_000, "uosmo"),
                ],
                2,
            )
            .unwrap();
        let admin = &accs[0];
        let new_admin = &accs[1];

        let wasm = Wasm::new(&app);
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, admin)
            .unwrap()
            .data
            .code_id;
        assert_eq!(code_id, 1);

        // initialize admins and check if the state is correct
        let init_admins = vec![admin.address()];
        let contract_addr = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    admins: init_admins.clone(),
                    mutable: true,
                },
                Some(&admin.address()),
                None,
                &[],
                admin,
            )
            .unwrap()
            .data
            .address;
        let admin_list = wasm
            .query::<QueryMsg, AdminListResponse>(&contract_addr, &QueryMsg::AdminList {})
            .unwrap();
        assert_eq!(admin_list.admins, init_admins);
        assert!(admin_list.mutable);

        // update admin and check again
        let new_admins = vec![new_admin.address()];
        wasm.execute::<ExecuteMsg>(
            &contract_addr,
            &ExecuteMsg::UpdateAdmins {
                admins: new_admins.clone(),
            },
            &[],
            admin,
        )
        .unwrap();

        let admin_list = wasm
            .query::<QueryMsg, AdminListResponse>(&contract_addr, &QueryMsg::AdminList {})
            .unwrap();

        assert_eq!(admin_list.admins, new_admins);
        assert!(admin_list.mutable);
    }

    #[test]
    fn test_custom_fee() {
        let app = OsmosisTestApp::default();
        let initial_balance = 1_000_000_000_000;
        let alice = app.init_account(&coins(initial_balance, "uosmo")).unwrap();
        let bob = app.init_account(&coins(initial_balance, "uosmo")).unwrap();

        let amount = Coin::new(1_000_000, "uosmo");
        let gas_limit = 100_000_000;

        // use FeeSetting::Auto by default, so should not equal newly custom fee setting
        let wasm = Wasm::new(&app);
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let res = wasm.store_code(&wasm_byte_code, None, &alice).unwrap();

        assert_ne!(res.gas_info.gas_wanted, gas_limit);

        //update fee setting
        let bob = bob.with_fee_setting(FeeSetting::Custom {
            amount: amount.clone(),
            gas_limit,
        });
        let res = wasm.store_code(&wasm_byte_code, None, &bob).unwrap();

        let bob_balance = Bank::new(&app)
            .query_all_balances(&QueryAllBalancesRequest {
                address: bob.address(),
                pagination: None,
            })
            .unwrap()
            .balances
            .into_iter()
            .find(|c| c.denom == "uosmo")
            .unwrap()
            .amount
            .parse::<u128>()
            .unwrap();

        assert_eq!(res.gas_info.gas_wanted, gas_limit);
        assert_eq!(bob_balance, initial_balance - amount.amount.u128());
    }

    #[test]
    fn test_querier_impl() {
        let app = OsmosisTestApp::default();
        let accs = app
            .init_accounts(
                &[
                    Coin::new(1_000_000_000_000, "uatom"),
                    Coin::new(1_000_000_000_000, "uosmo"),
                ],
                2,
            )
            .unwrap();
        let acc1 = &accs[0];
        let acc2 = &accs[1];
        let gamm = Gamm::new(&app);

        // Create pool
        let pool_liquidity = vec![Coin::new(1_000, "uatom"), Coin::new(1_000, "uosmo")];
        let _pool_id = gamm
            .create_basic_pool(&pool_liquidity, &acc1)
            .unwrap()
            .data
            .pool_id;

        let querier = QuerierWrapper::<Empty>::new(&app);

        // Bank::Balance
        let balance = querier.query_balance(acc2.address(), "uosmo").unwrap();
        assert_eq!(balance.amount, Uint128::new(1_000_000_000_000));

        // Bank::Supply
        let supply = querier.query_supply("uosmo").unwrap();
        assert!(supply.amount > Uint128::zero());

        // TODO: fix this
        // Stargate Query
        // let msg = QueryPoolRequest { pool_id };
        // let mut buf = Vec::new();
        // QueryPoolRequest::encode(&msg, &mut buf).unwrap();
        // let res: Vec<u8> = querier
        //     .query(&QueryRequest::Stargate {
        //         path: "/osmosis.gamm.v1beta1.Query/Pool".into(),
        //         data: buf.into(),
        //     })
        //     .unwrap();
        // let res = QueryPoolResponse::decode(&mut res.as_slice()).unwrap();
        // let pool = gamm::v1beta1::Pool::decode(res.pool.unwrap().value.as_slice()).unwrap();
        // assert_eq!(pool.id, pool_id);
    }
}
