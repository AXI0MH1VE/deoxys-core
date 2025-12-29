// Copyright (c) 2025 Axiom Hive. All Rights Reserved.
// Author: Alexis Adams <sovereign@axiomhive.net>
// SPDX-License-Identifier: Proprietary

mod rik;
mod invariants;
mod crypto;
mod substrate;

use crate::rik::RikEngine;
use crate::substrate::SovereignState;
use log::{info, error, warn};
use std::time::{Duration, Instant};
use std::io::{self, Write};
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

    info!(">> SYSTEM ACTIVE: Entering Human-Supervised RIK Loop");
    info!(">> HUMAN-IN-THE-LOOP: Manual approval required for each cycle execution");

    // 3. The Human-Supervised Loop
    let mut cycle_count = 0u64;
    loop {
        cycle_count += 1;
        
        // HUMAN APPROVAL GATE: Require explicit human approval before execution
        info!("\n=== CYCLE {} APPROVAL REQUEST ===", cycle_count);
        print!("Approve cycle execution? (y/n): ");
        io::stdout().flush().unwrap();
        
        let mut approval = String::new();
        io::stdin().read_line(&mut approval).unwrap();
        let approval = approval.trim().to_lowercase();
        
        if approval != "y" && approval != "yes" {
            warn!("!! CYCLE {} DENIED: Human operator rejected execution", cycle_count);
            info!(">> Enter 'exit' to terminate system, or any other key to continue to next approval cycle:");
            
            let mut next_action = String::new();
            io::stdin().read_line(&mut next_action).unwrap();
            if next_action.trim().to_lowercase() == "exit" {
                info!(">> SYSTEM SHUTDOWN: Terminated by human operator");
                break;
            }
            continue;
        }
        
        info!(">> CYCLE {} APPROVED: Executing with human oversight...", cycle_count);
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
    
    Ok(())
}
