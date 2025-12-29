// Copyright (c) 2025 Axiom Hive. All Rights Reserved.
// Author: Alexis Adams <sovereign@axiomhive.net>
// SPDX-License-Identifier: Proprietary

mod rik;
mod invariants;
mod crypto;
mod substrate;

use crate::rik::{RikEngine, OperatorIntent};
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
        
        // Capture operator's intent for this cycle with validation loop
        let operator_intent = loop {
            print!("Specify output bounds - Min value (default: -1.0): ");
            io::stdout().flush().unwrap();
            let mut min_input = String::new();
            io::stdin().read_line(&mut min_input).unwrap();
            let min_bound: f64 = match min_input.trim().parse::<f64>() {
                Ok(val) if val.is_finite() => val,
                Ok(_) => {
                    error!("!! Invalid min bound: must be a finite number (not NaN or infinity), using default: -1.0");
                    -1.0
                }
                Err(_) if min_input.trim().is_empty() => -1.0,
                Err(_) => {
                    warn!("!! Invalid min bound input, using default: -1.0");
                    -1.0
                }
            };
            
            print!("Specify output bounds - Max value (default: 1.0): ");
            io::stdout().flush().unwrap();
            let mut max_input = String::new();
            io::stdin().read_line(&mut max_input).unwrap();
            let max_bound: f64 = match max_input.trim().parse::<f64>() {
                Ok(val) if val.is_finite() => val,
                Ok(_) => {
                    error!("!! Invalid max bound: must be a finite number (not NaN or infinity), using default: 1.0");
                    1.0
                }
                Err(_) if max_input.trim().is_empty() => 1.0,
                Err(_) => {
                    warn!("!! Invalid max bound input, using default: 1.0");
                    1.0
                }
            };
            
            print!("Intent description (default: 'Standard bounds'): ");
            io::stdout().flush().unwrap();
            let mut desc_input = String::new();
            io::stdin().read_line(&mut desc_input).unwrap();
            let description = if desc_input.trim().is_empty() {
                "Standard bounds".to_string()
            } else {
                desc_input.trim().to_string()
            };
            
            match OperatorIntent::new(min_bound, max_bound, description) {
                Ok(intent) => {
                    info!(">> Operator Intent Captured: {} (bounds: [{}, {}])", 
                          intent.description, 
                          intent.min_bound, 
                          intent.max_bound);
                    break intent;
                }
                Err(e) => {
                    error!("!! INVALID BOUNDS: {}", e);
                    warn!("!! Please re-enter the output bounds.\n");
                    continue;
                }
            }
        };
        
        print!("Approve cycle execution with these bounds? (y/n): ");
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

        match engine.execute_cycle(&operator_intent).await {
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
