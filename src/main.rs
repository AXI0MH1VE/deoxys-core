mod rik;
mod invariants;
mod crypto;
mod substrate;

use crate::rik::RikEngine;
use crate::substrate::SovereignState;
use log::{info, error};
use std::time::{Duration, Instant};
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 0. Environment Setup
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    info!(">> DEOXYS HEGEMONY FORGE v2.0 - INITIALIZING");
    info!(">> ROOT AUTHORITY: Alexis Adams (C == XNXAlexis)");

    // 1. Initialize Sovereign Substrate (The Invariant)
    let substrate = SovereignState::new("C_EQUALS_XNXALEXIS_ROOT");
    if !substrate.verify_integrity() {
        error!("!! INTEGRITY VIOLATION: Substrate compromised. Halting.");
        std::process::exit(1);
    }

    // 2. Boot RIK Engine
    let mut engine = RikEngine::new(substrate);
    let cycle_target = Duration::from_millis(50); // 20Hz

    info!(">> SYSTEM ACTIVE: Entering 20Hz RIK God Loop");

    // 3. The God Loop
    loop {
        let cycle_start = Instant::now();

        match engine.execute_cycle().await {
            Ok(receipt) => {
                info!("<< CYCLE COMPLETE: Hash={} | Latency={:?}", receipt.hash, cycle_start.elapsed());
            }
            Err(e) => {
                error!("!! CYCLE FAILURE: Invariant breach detected: {}", e);
                // In production, this triggers the "Irreversible Covenant" lockdown
                // For now, we panic to enforce the Zero Entropy Law (Fail-Stop)
                panic!("Zero Entropy Law Violation: System Halted.");
            }
        }

        let elapsed = cycle_start.elapsed();
        if elapsed < cycle_target {
            time::sleep(cycle_target - elapsed).await;
        } else {
            // Log overrun but do not yield; maintain pressure
            info!("!! CYCLE OVERRUN: {:?} > 50ms", elapsed);
        }
    }
}
