use aquarium::{
    utils::{parse_code_id, parse_instantiated_address},
    ContractInstance, Env, Executor, Querier,
};

use interface::InstantiateMsg;

#[aquarium::task]
async fn deploy(env: &mut Env) {
    let bytecode = include_bytes!("../../artifacts/icatest_controller-aarch64.wasm");
    let hash = env
        .executor
        .store_code(bytecode.to_vec(), None)
        .await
        .unwrap();
    println!("Waiting for storecode hash {hash}");
    let receipt = env.executor.wait_for_transaction(hash).await.unwrap();
    let code_id = parse_code_id(&receipt).unwrap();
    println!("Storecode code id {code_id}");
    env.refs.add_code_id("icatest", code_id);

    let hash = env
        .executor
        .instantiate(
            code_id,
            &InstantiateMsg {},
            vec![],
            Some("icatest".to_string()),
            None,
            None,
        )
        .await
        .unwrap();
    println!("Waiting for instantiate hash {hash}");
    let receipt = env.executor.wait_for_transaction(hash).await.unwrap();
    let contract_addr = parse_instantiated_address(&receipt).unwrap();
    println!("Instantiated contract address {contract_addr}");
    let instance = ContractInstance::new(code_id, contract_addr);
    env.refs.contracts.get_mut("icatest").unwrap().instances = vec![instance];
}
