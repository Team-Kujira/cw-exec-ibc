use aquarium::{Env, Executor, Querier};

use interface::ExecuteMsg;

#[aquarium::task]
async fn register(env: &mut Env) {
    let msg = ExecuteMsg::CreateAccount {
        conn_id: "connection-0".to_string(),
        acc_id: "1".to_string(),
        version: "".to_string(),
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
