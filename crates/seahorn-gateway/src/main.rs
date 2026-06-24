//! seahorn-gateway — Horizon TAP v2 (GraphTally) payment layer in front of the Seahorn
//! query layer (PostgREST over the indexer's Postgres sink).
//!
//! All payment machinery (receipt validation, RAV aggregation, on-chain collection,
//! persistence, the TAP-gated reverse proxy) lives in `horizon-core`. This binary loads
//! config and hands off: a consumer sends a signed `TAP-Receipt` header, the gateway
//! verifies + meters it and proxies the request to `backend.upstream_url`.
//!
//! The Seahorn INDEXER (substrate → handler → sink) is a separate binary and is
//! unaffected by this gateway.

use horizon_core::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "seahorn_gateway=info,horizon_core=info".into()),
        )
        .init();

    let config = Config::load()?;
    tracing::info!(
        upstream = %config.backend.upstream_url,
        data_service = %config.tap.data_service_address,
        "seahorn-gateway starting — Solana data on Horizon"
    );

    horizon_core::run(config).await
}
