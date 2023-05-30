use aquarium::{Env, Executor, Querier};

use cosmwasm_std::Coin;
use interface::ExecuteMsg;

#[aquarium::task]
async fn delegate(env: &mut Env) {
    let msg = ExecuteMsg::SendDelegateTx {
        conn_id: "connection-0".to_string(),
        acc_id: "1".to_string(),
        validator: "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn".to_string(),
        // delegator: "cosmos12tmv3chlulnk3e0j6mm6gp964qjurk7cg074yv3jtnsm92557t0qda784w".to_string(),
        amount: Coin::new(1000u128, "stake")
    };

    let addr = env.refs.contracts.get("icatest").unwrap().instances[0]
        .address
        .clone();

    let hash = env
        .executor
        .execute_smart(addr, &msg, vec![], None)
        .await
        .unwrap();

    println!("Waiting for execute hash {hash}");
    let receipt = env.executor.wait_for_transaction(hash).await.unwrap();
    println!("Executed contract address {receipt}");
}
