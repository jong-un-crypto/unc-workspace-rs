/// URL to the Pagoda API to use for testnet.
pub const PAGODA_TESTNET_RPC_URL: &str = "https://unc-testnet.api.pagoda.co/rpc/v1/";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // `UNC_RPC_API_KEY="xxx" cargo run --package examples --example custom_network`
    if let Ok(val) = std::env::var("UNC_RPC_API_KEY") {
        // Reference to what can be called by this network: https://docs.pagoda.co/endpoints
        let worker = unc_workspaces::custom(PAGODA_TESTNET_RPC_URL)
            .api_key(&val)
            .await?;
        let res = worker.view_block().await?;

        assert!(res.height() > 0);
        return Ok(());
    }

    // skip the test
    println!("UNC_RPC_API_KEY is not set, skipping the example");
    Ok(())
}
