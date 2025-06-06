// Copyright 2019 Conflux Foundation. All rights reserved.
// Conflux is free software and distributed under GNU General Public License.
// See http://www.gnu.org/licenses/

use super::RpcAddress;
use cfx_addr::Network;
use cfx_types::{H256, U256};
use primitives::Account as PrimitiveAccount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    // This field isn't part of Account RLP but is helpful for debugging.
    pub address: RpcAddress,
    pub balance: U256,
    pub nonce: U256,
    pub code_hash: H256,
    pub staking_balance: U256,
    pub collateral_for_storage: U256,
    pub accumulated_interest_return: U256,
    pub admin: RpcAddress,
}

impl Account {
    pub fn try_from(
        account: PrimitiveAccount, network: Network,
    ) -> Result<Self, String> {
        let collateral_for_storage = account.collateral_for_storage
            + account
                .sponsor_info
                .storage_points
                .as_ref()
                .map_or(U256::zero(), |x| x.used);
        Ok(Self {
            address: RpcAddress::try_from_h160(
                account.address().address,
                network,
            )?,
            balance: account.balance.into(),
            nonce: account.nonce.into(),
            code_hash: account.code_hash.into(),
            staking_balance: account.staking_balance.into(),
            collateral_for_storage,
            accumulated_interest_return: account
                .accumulated_interest_return
                .into(),
            admin: RpcAddress::try_from_h160(account.admin, network)?,
        })
    }
}
