use cosmrs::{
    crypto::{secp256k1::SigningKey, PublicKey},
    AccountId,
};
use cosmwasm_std::Coin;

const OSMO_ADDRESS_PREFIX: &str = "osmo";

pub trait Account {
    fn public_key(&self) -> PublicKey;
    fn address(&self) -> String;
    fn account_id(&self) -> AccountId;
}
pub struct SigningAccount {
    prefix: String,
    signing_key: SigningKey,
    fee_setting: FeeSetting,
}

impl SigningAccount {
    pub fn new(prefix: String, signing_key: SigningKey, fee_setting: FeeSetting) -> Self {
        SigningAccount {
            prefix,
            signing_key,
            fee_setting,
        }
    }

    pub fn fee_setting(&self) -> &FeeSetting {
        &self.fee_setting
    }

    pub fn with_fee_setting(self, fee_setting: FeeSetting) -> Self {
        Self {
            prefix: self.prefix,
            signing_key: self.signing_key,
            fee_setting,
        }
    }
}

impl Account for SigningAccount {
    fn public_key(&self) -> PublicKey {
        self.signing_key.public_key()
    }

    fn address(&self) -> String {
        self.account_id().to_string()
    }
    fn account_id(&self) -> AccountId {
        self.public_key()
            .account_id(&self.prefix)
            .expect("prefix is constant and must valid")
    }
}

impl SigningAccount {
    pub fn signing_key(&'_ self) -> &'_ SigningKey {
        &self.signing_key
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonSigningAccount {
    public_key: PublicKey,
}

impl From<PublicKey> for NonSigningAccount {
    fn from(public_key: PublicKey) -> Self {
        NonSigningAccount { public_key }
    }
}
impl From<SigningAccount> for NonSigningAccount {
    fn from(signing_account: SigningAccount) -> Self {
        NonSigningAccount {
            public_key: signing_account.public_key(),
        }
    }
}

impl Account for NonSigningAccount {
    fn public_key(&self) -> PublicKey {
        self.public_key
    }

    fn address(&self) -> String {
        self.account_id().to_string()
    }
    fn account_id(&self) -> AccountId {
        self.public_key()
            .account_id(OSMO_ADDRESS_PREFIX)
            .expect("ADDRESS_PREFIX is constant and must valid")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeeSetting {
    Auto {
        gas_price: Coin,
        gas_adjustment: f64,
    },
    Custom {
        amount: Coin,
        gas_limit: u64,
    },
}
