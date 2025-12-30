/*
 * Copyright (c) 2025 - Cowboy AI, LLC.
 */

//! Relationship Service Binary
//!
//! NATS-connected service for the relationship domain.

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());

    tracing::info!("Starting relationship-service");
    tracing::info!("NATS URL: {}", nats_url);

    // TODO: Implement NATS connection and command handler
    // For now, just a placeholder that demonstrates the library compiles

    tracing::info!("Relationship service started (placeholder)");

    // Keep running
    tokio::signal::ctrl_c().await?;

    tracing::info!("Shutting down relationship-service");
    Ok(())
}
