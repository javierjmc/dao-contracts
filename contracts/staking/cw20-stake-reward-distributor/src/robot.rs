use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::{BasicApp, Executor};

use crate::tests::{
    get_balance_cw20, get_info, instantiate_cw20, instantiate_distributor, instantiate_staking,
    OWNER,
};
use crate::{
    msg::{ExecuteMsg, InfoResponse, InstantiateMsg, MigrateMsg, QueryMsg},
    state::Config,
    ContractError,
};

pub enum AssertContract {
    Staking,
    Distributor,
    Cw20,
}

pub struct StakingRobot {
    app: BasicApp,
    cw20_addr: Option<Addr>,
    staking_addr: Option<Addr>,
    distributor_addr: Option<Addr>,
}

impl StakingRobot {
    pub fn new(app: BasicApp) -> Self {
        Self {
            app,
            cw20_addr: None,
            staking_addr: None,
            distributor_addr: None,
        }
    }

    pub fn instantiate_contracts(
        &mut self,
        cw20_coin: cw20::Cw20Coin,
        distributor_instantiate_msg: InstantiateMsg,
    ) -> &mut Self {
        self.cw20_addr = Some(instantiate_cw20(&mut self.app, vec![cw20_coin]));
        self.staking_addr = Some(instantiate_staking(
            &mut self.app,
            self.cw20_addr.unwrap().clone(),
        ));
        self.distributor_addr = Some(instantiate_distributor(
            &mut self.app,
            distributor_instantiate_msg,
        ));
        self
    }

    pub fn load_distributor(&mut self, amount: u128) -> &mut Self {
        let msg = cw20::Cw20ExecuteMsg::Transfer {
            recipient: distributor_addr.to_string(),
            amount: Uint128::from(amount),
        };

        self.app
            .execute_contract(
                Addr::unchecked(OWNER),
                self.cw20_addr.unwrap().clone(),
                &msg,
                &[],
            )
            .unwrap();

        self
    }

    pub fn distribute_rewards(&mut self, update_blockheight: i64) -> &mut Self {
        self.app
            .update_block(|mut block| block.height += update_blockheight);
        self.app
            .execute_contract(
                Addr::unchecked(OWNER),
                self.distributor_addr.unwrap().clone(),
                &ExecuteMsg::Distribute {},
                &[],
            )
            .unwrap();

        self
    }

    pub fn assert_balance(&mut self, contract: AssertContract, amount: u128) -> &mut Self {
        let staking_balance = get_balance_cw20(
            &self.app,
            self.cw20_addr.unwrap().clone(),
            self.staking_addr.unwrap().clone(),
        );
        assert_eq!(staking_balance, Uint128::new(amount));
        self
    }

    pub fn get_distributor_info(&mut self, info: impl Fn((&mut Self, InfoResponse))) -> &mut Self {
        info((
            self,
            get_info(&self.app, self.distributor_addr.unwrap().clone()),
        ));
        self
    }

    pub fn assert_distributor_info(&mut self, amount: u128) -> &mut Self {
        let staking_balance = get_balance_cw20(
            &self.app,
            self.cw20_addr.unwrap().clone(),
            self.staking_addr.unwrap().clone(),
        );
        assert_eq!(staking_balance, Uint128::new(amount));
        self
    }

    pub fn assert_no_payout(&mut self) -> &mut Self {
        let err: ContractError = self
            .app
            .execute_contract(
                Addr::unchecked(OWNER),
                self.distributor_addr.unwrap().clone(),
                &ExecuteMsg::Distribute {},
                &[],
            )
            .unwrap_err()
            .downcast()
            .unwrap();
        assert!(matches!(err, ContractError::RewardsDistributedForBlock {}));
        self
    }
}
