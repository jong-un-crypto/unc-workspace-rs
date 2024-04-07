const STATUS_MSG_WASM_FILEPATH: &str = "./examples/res/status_message.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = unc_workspaces::sandbox().await?;
    let wasm = std::fs::read(STATUS_MSG_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    let outcome = contract
        .call("set_status")
        .args_json(serde_json::json!({
            "message": "hello_world",
        }))
        .transact()
        .await?;

    let block_reference = {
        let hash = outcome.outcome().block_hash;
        unc_primitives::types::BlockReference::BlockId(unc_primitives::types::BlockId::Hash(
            unc_primitives::hash::CryptoHash(hash.0),
        ))
    };

    // NOTE: this API is under the "experimental" flag and no guarantees are given.
    let protocol_config = worker.protocol_config(block_reference).await?;

    // Example output:
    //
    // ProtocolConfig ProtocolConfigView {
    //     protocol_version: 60,
    //     genesis_time: 2023-08-22T10:07:46.377793Z,
    //     chain_id: "test-chain-trbrQ",
    //     genesis_height: 0,
    //     num_block_producer_seats: 50,
    //     num_block_producer_seats_per_shard: [
    //         50,
    //     ],
    //     avg_hidden_validator_seats_per_shard: [
    //         0,
    //     ],
    //     dynamic_resharding: false,
    //     protocol_upgrade_stake_threshold: Ratio {
    //         numer: 4,
    //         denom: 5,
    //     },
    //     epoch_length: 500,
    //     gas_limit: 1000000000000000,
    //     min_gas_price: 100000000,
    //     max_gas_price: 10000000000000000000000,
    //     block_producer_kickout_threshold: 90,
    //     chunk_producer_kickout_threshold: 90,
    //     online_min_threshold: Ratio {
    //         numer: 9,
    //         denom: 10,
    //     },
    //     online_max_threshold: Ratio {
    //         numer: 99,
    //         denom: 100,
    //     },
    //     gas_price_adjustment_rate: Ratio {
    //         numer: 1,
    //         denom: 100,
    //     },
    //     runtime_config: RuntimeConfigView {
    //         storage_amount_per_byte: 10000000000000000000,
    //         transaction_costs: RuntimeFeesConfigView {
    //             action_receipt_creation_config: Fee {
    //                 send_sir: 108059500000,
    //                 send_not_sir: 108059500000,
    //                 execution: 108059500000,
    //             },
    //             data_receipt_creation_config: DataReceiptCreationConfigView {
    //                 base_cost: Fee {
    //                     send_sir: 36486732312,
    //                     send_not_sir: 36486732312,
    //                     execution: 36486732312,
    //                 },
    //                 cost_per_byte: Fee {
    //                     send_sir: 17212011,
    //                     send_not_sir: 17212011,
    //                     execution: 17212011,
    //                 },
    //             },
    //             action_creation_config: ActionCreationConfigView {
    //                 create_account_cost: Fee {
    //                     send_sir: 3850000000000,
    //                     send_not_sir: 3850000000000,
    //                     execution: 3850000000000,
    //                 },
    //                 deploy_contract_cost: Fee {
    //                     send_sir: 184765750000,
    //                     send_not_sir: 184765750000,
    //                     execution: 184765750000,
    //                 },
    //                 deploy_contract_cost_per_byte: Fee {
    //                     send_sir: 6812999,
    //                     send_not_sir: 6812999,
    //                     execution: 64572944,
    //                 },
    //                 function_call_cost: Fee {
    //                     send_sir: 2319861500000,
    //                     send_not_sir: 2319861500000,
    //                     execution: 2319861500000,
    //                 },
    //                 function_call_cost_per_byte: Fee {
    //                     send_sir: 2235934,
    //                     send_not_sir: 2235934,
    //                     execution: 2235934,
    //                 },
    //                 transfer_cost: Fee {
    //                     send_sir: 115123062500,
    //                     send_not_sir: 115123062500,
    //                     execution: 115123062500,
    //                 },
    //                 stake_cost: Fee {
    //                     send_sir: 141715687500,
    //                     send_not_sir: 141715687500,
    //                     execution: 102217625000,
    //                 },
    //                 add_key_cost: AccessKeyCreationConfigView {
    //                     full_access_cost: Fee {
    //                         send_sir: 101765125000,
    //                         send_not_sir: 101765125000,
    //                         execution: 101765125000,
    //                     },
    //                     function_call_cost: Fee {
    //                         send_sir: 102217625000,
    //                         send_not_sir: 102217625000,
    //                         execution: 102217625000,
    //                     },
    //                     function_call_cost_per_byte: Fee {
    //                         send_sir: 1925331,
    //                         send_not_sir: 1925331,
    //                         execution: 1925331,
    //                     },
    //                 },
    //                 delete_key_cost: Fee {
    //                     send_sir: 94946625000,
    //                     send_not_sir: 94946625000,
    //                     execution: 94946625000,
    //                 },
    //                 delete_account_cost: Fee {
    //                     send_sir: 147489000000,
    //                     send_not_sir: 147489000000,
    //                     execution: 147489000000,
    //                 },
    //                 delegate_cost: Fee {
    //                     send_sir: 200000000000,
    //                     send_not_sir: 200000000000,
    //                     execution: 200000000000,
    //                 },
    //             },
    //             storage_usage_config: StorageUsageConfigView {
    //                 num_bytes_account: 100,
    //                 num_extra_bytes_record: 40,
    //             },
    //             burnt_gas_reward: Ratio {
    //                 numer: 3,
    //                 denom: 10,
    //             },
    //             pessimistic_gas_price_inflation_ratio: Ratio {
    //                 numer: 103,
    //                 denom: 100,
    //             },
    //         },
    //         wasm_config: VMConfigView {
    //             ext_costs: ExtCostsConfigView {
    //                 base: 264768111,
    //                 contract_loading_base: 35445963,
    //                 contract_loading_bytes: 216750,
    //                 read_memory_base: 2609863200,
    //                 read_memory_byte: 3801333,
    //                 write_memory_base: 2803794861,
    //                 write_memory_byte: 2723772,
    //                 read_register_base: 2517165186,
    //                 read_register_byte: 98562,
    //                 write_register_base: 2865522486,
    //                 write_register_byte: 3801564,
    //                 utf8_decoding_base: 3111779061,
    //                 utf8_decoding_byte: 291580479,
    //                 utf16_decoding_base: 3543313050,
    //                 utf16_decoding_byte: 163577493,
    //                 sha256_base: 4540970250,
    //                 sha256_byte: 24117351,
    //                 keccak256_base: 5879491275,
    //                 keccak256_byte: 21471105,
    //                 keccak512_base: 5811388236,
    //                 keccak512_byte: 36649701,
    //                 ripemd160_base: 853675086,
    //                 ripemd160_block: 680107584,
    //                 ed25519_verify_base: 210000000000,
    //                 ed25519_verify_byte: 9000000,
    //                 ecrecover_base: 278821988457,
    //                 log_base: 3543313050,
    //                 log_byte: 13198791,
    //                 storage_write_base: 64196736000,
    //                 storage_write_key_byte: 70482867,
    //                 storage_write_value_byte: 31018539,
    //                 storage_write_evicted_byte: 32117307,
    //                 storage_read_base: 56356845750,
    //                 storage_read_key_byte: 30952533,
    //                 storage_read_value_byte: 5611005,
    //                 storage_remove_base: 53473030500,
    //                 storage_remove_key_byte: 38220384,
    //                 storage_remove_ret_value_byte: 11531556,
    //                 storage_has_key_base: 54039896625,
    //                 storage_has_key_byte: 30790845,
    //                 storage_iter_create_prefix_base: 0,
    //                 storage_iter_create_prefix_byte: 0,
    //                 storage_iter_create_range_base: 0,
    //                 storage_iter_create_from_byte: 0,
    //                 storage_iter_create_to_byte: 0,
    //                 storage_iter_next_base: 0,
    //                 storage_iter_next_key_byte: 0,
    //                 storage_iter_next_value_byte: 0,
    //                 touching_trie_node: 16101955926,
    //                 read_cached_trie_node: 2280000000,
    //                 promise_and_base: 1465013400,
    //                 promise_and_per_promise: 5452176,
    //                 promise_return: 560152386,
    //                 validator_stake_base: 911834726400,
    //                 validator_total_stake_base: 911834726400,
    //                 contract_compile_base: 0,
    //                 contract_compile_bytes: 0,
    //                 alt_bn128_g1_multiexp_base: 713000000000,
    //                 alt_bn128_g1_multiexp_element: 320000000000,
    //                 alt_bn128_g1_sum_base: 3000000000,
    //                 alt_bn128_g1_sum_element: 5000000000,
    //                 alt_bn128_pairing_check_base: 9686000000000,
    //                 alt_bn128_pairing_check_element: 5102000000000,
    //             },
    //             grow_mem_cost: 1,
    //             regular_op_cost: 822756,
    //             limit_config: VMLimitConfig {
    //                 max_gas_burnt: 300000000000000,
    //                 max_stack_height: 16384,
    //                 contract_prepare_version: V1,
    //                 initial_memory_pages: 1024,
    //                 max_memory_pages: 2048,
    //                 registers_memory_limit: 1073741824,
    //                 max_register_size: 104857600,
    //                 max_number_registers: 100,
    //                 max_number_logs: 100,
    //                 max_total_log_length: 16384,
    //                 max_total_prepaid_gas: 300000000000000,
    //                 max_actions_per_receipt: 100,
    //                 max_number_bytes_method_names: 2000,
    //                 max_length_method_name: 256,
    //                 max_arguments_length: 4194304,
    //                 max_length_returned_data: 4194304,
    //                 max_contract_size: 4194304,
    //                 max_transaction_size: 4194304,
    //                 max_length_storage_key: 2048,
    //                 max_length_storage_value: 4194304,
    //                 max_promises_per_function_call_action: 1024,
    //                 max_number_input_data_dependencies: 128,
    //                 max_functions_number_per_contract: Some(
    //                     10000,
    //                 ),
    //                 wasmer2_stack_limit: 204800,
    //                 max_locals_per_contract: Some(
    //                     1000000,
    //                 ),
    //                 account_id_validity_rules_version: V1,
    //             },
    //         },
    //         account_creation_config: AccountCreationConfigView {
    //             min_allowed_top_level_account_length: 32,
    //             registrar_account_id: AccountId(
    //                 "registrar",
    //             ),
    //         },
    //     },
    //     transaction_validity_period: 100,
    //     protocol_reward_rate: Ratio {
    //         numer: 1,
    //         denom: 10,
    //     },
    //     max_inflation_rate: Ratio {
    //         numer: 1,
    //         denom: 20,
    //     },
    //     num_blocks_per_year: 31536000,
    //     protocol_treasury_account: AccountId(
    //         "test.unc",
    //     ),
    //     fishermen_threshold: 10000000000000000000000000,
    //     minimum_stake_divisor: 10,
    // }
    println!("ProtocolConfig {protocol_config:#?}");
    Ok(())
}
