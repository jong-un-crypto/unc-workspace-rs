#![recursion_limit = "256"]
use unc_gas::UncGas;
use unc_workspaces::types::UncToken;
use unc_workspaces::{Contract, DevNetwork, Worker};
use test_log::test;

async fn init(worker: &Worker<impl DevNetwork>) -> anyhow::Result<Contract> {
    let contract = worker
        .dev_deploy(include_bytes!("../../examples/res/fungible_token.wasm"))
        .await?;

    contract
        .call("new_default_meta")
        .args_json(serde_json::json!({
            "owner_id": contract.id(),
            "total_supply": UncToken::from_unc(1_000_000_000),
        }))
        .transact()
        .await?
        .into_result()?;

    Ok(contract)
}

#[test(tokio::test)]
async fn test_empty_args_error() -> anyhow::Result<()> {
    let worker = unc_workspaces::sandbox().await?;
    let contract = init(&worker).await?;

    let res = contract
        .call("storage_unregister")
        .max_gas()
        .deposit(UncToken::from_attounc(1))
        .transact()
        .await?
        .into_result();

    if let Some(execution_err) = res.err() {
        assert!(format!("{}", execution_err).contains("Failed to deserialize input from JSON"));
        assert!(
            execution_err.total_gas_burnt > UncGas::from_gas(0),
            "Gas is still burnt for transaction although inputs are incorrect"
        );
    } else {
        panic!("Expected execution to error out");
    }

    Ok(())
}

#[test(tokio::test)]
async fn test_optional_args_present() -> anyhow::Result<()> {
    let worker = unc_workspaces::sandbox().await?;
    let contract = init(&worker).await?;

    let res = contract
        .call("storage_unregister")
        .args_json(serde_json::json!({
            "force": true
        }))
        .max_gas()
        .deposit(UncToken::from_attounc(1))
        .transact()
        .await?;
    assert!(res.json::<bool>()?);

    Ok(())
}
