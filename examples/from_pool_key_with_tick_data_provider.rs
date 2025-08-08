//! Example demonstrating pool creation with tick data provider and swap simulation
//!
//! # Prerequisites
//! - Environment variable MAINNET_RPC_URL must be set
//! - Requires the "extensions" feature
//!
//! # Note
//! This example uses mainnet block 17000000 for consistent results

use alloy::{
    eips::BlockId,
    providers::{ProviderBuilder},
    transports::http::reqwest::Url,
};
use alloy_primitives::{address, U160};
use uniswap_sdk_core::{prelude::*, token};
use uniswap_v3_sdk::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let rpc_url: Url = std::env::var("BASE_RPC_URL").unwrap().parse().unwrap();
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    let block_id = BlockId::from(28502265);
    // let wbtc = token!(1, "2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599", 8, "WBTC");
    let weth = WETH9::on_chain(8453).unwrap();
    let usdt = token!(8453, "fde4C96c8593536E31F229EA8f37b2ADa2699bb2", 6, "USDT");

    // Create a pool with a tick map data provider
    let pool = Pool::<EphemeralTickMapDataProvider>::from_sqrt_ratio(
        8453,
        address!("9785eF59E2b499fB741674ecf6fAF912Df7b3C1b"),
        weth.address(),
        usdt.address(),
        FeeAmount::LOW_400,
        U160::from(3372497486071350466090442u128),
        100,
        100,
        provider.clone(),
        Some(block_id),
    )
    .await
    .unwrap();
    // Get the output amount from the pool
    let amount_in = CurrencyAmount::from_raw_amount(weth.clone(), 100000000000000000u128).unwrap();
    let local_amount_out = pool.get_output_amount(&amount_in, None).unwrap();
    let local_amount_out = local_amount_out.quotient();
    println!("Local amount out: {}", local_amount_out);

    // // Get the output amount from the quoter
    // let route = Route::new(vec![pool], wbtc, weth);
    // let params = quote_call_parameters(&route, &amount_in, TradeType::ExactInput, None);
    // let tx = TransactionRequest::default()
    //     .to(*QUOTER_ADDRESSES.get(&1).unwrap())
    //     .input(params.calldata.into());
    // let res = provider.call(&tx).block(block_id).await.unwrap();
    // let amount_out = IQuoter::quoteExactInputSingleCall::abi_decode_returns(res.as_ref(), true)
    //     .unwrap()
    //     .amountOut;
    // println!("Quoter amount out: {}", amount_out);
    //
    // // Compare local calculation with on-chain quoter to ensure accuracy
    // assert_eq!(U256::from_big_int(local_amount_out), amount_out);
}
