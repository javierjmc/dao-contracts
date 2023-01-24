use cosmwasm_std::Uint128;
use cw_multi_test::App;

use crate::robot::{AssertContract, StakingRobot};
use crate::tests::OWNER;
use crate::{
    msg::{ExecuteMsg, InfoResponse, InstantiateMsg, MigrateMsg, QueryMsg},
    state::Config,
    ContractError,
};

#[test]
fn test_distribute_with_robot() {
    let mut robot = StakingRobot::new(App::default());

    robot
        .instantiate_contracts(
            cw20::Cw20Coin {
                address: OWNER.to_string(),
                amount: Uint128::from(1000u64),
            },
            InstantiateMsg {
                owner: OWNER.to_string(),
                staking_addr: staking_addr.to_string(),
                reward_rate: Uint128::new(1),
                reward_token: cw20_addr.to_string(),
            },
        )
        .load_distributor(1000u128)
        .distribute_rewards(10)
        .assert_balance(AssertContract::Staking, 10)
        .get_distributor_info(|(robot, info)| {
            robot.assert_distributor_info(990);
        })
        .assert_distributor_info(990)
        .distribute_rewards(500)
        .assert_balance(Contract::Staking, 510)
        .get_distributor_info(|(robot, info)| {
            robot.assert_distributor_info(490);
        })
        .distribute_rewards(1000)
        .assert_balance(Contract::Staking, 1000)
        .get_distributor_info(|(robot, info)| {
            robot.assert_distributor_info(0);
        })
        .distribute_rewards(-2000)
        .assert_no_payout();
}
