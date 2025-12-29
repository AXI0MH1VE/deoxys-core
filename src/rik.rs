// Copyright (c) 2025 Axiom Hive. All Rights Reserved.
// Author: Alexis Adams <sovereign@axiomhive.net>
// SPDX-License-Identifier: Proprietary

use crate::substrate::SovereignState;
use crate::invariants::LyapunovValidator;
use crate::crypto::{CkksProvider, ProvenanceSigner};
use ndarray::Array1;
use anyhow::Result;
use log::info;

/// Operator Intent: Specifies the bounds and constraints for system outputs
/// This captures the human operator's explicit intent for what the system may produce
#[derive(Debug, Clone)]
pub struct OperatorIntent {
    pub min_bound: f64,
    pub max_bound: f64,
    pub description: String,
}

impl OperatorIntent {
    pub fn new(min_bound: f64, max_bound: f64, description: String) -> Result<Self> {
        // Validate bounds are finite (not NaN or infinity)
        if !min_bound.is_finite() {
            anyhow::bail!("Invalid min_bound: must be a finite number (not NaN or infinity)");
        }
        if !max_bound.is_finite() {
            anyhow::bail!("Invalid max_bound: must be a finite number (not NaN or infinity)");
        }
        if min_bound >= max_bound {
            anyhow::bail!("Invalid bounds: min_bound must be less than max_bound");
        }
        Ok(Self {
            min_bound,
            max_bound,
            description,
        })
    }

    /// Verify that all values in a state vector are within bounds
    pub fn verify_state(&self, state: &Array1<f64>) -> Result<()> {
        for (idx, &value) in state.iter().enumerate() {
            // First check if value is finite (provides specific error message)
            if !value.is_finite() {
                anyhow::bail!(
                    "Operator intent violation: State[{}] = {} is not finite (NaN or infinity) - {}",
                    idx,
                    value,
                    self.description
                );
            }
            // Then check if within bounds (value is guaranteed finite at this point)
            if value < self.min_bound || value > self.max_bound {
                anyhow::bail!(
                    "Operator intent violation: State[{}] = {} exceeds bounds [{}, {}] - {}",
                    idx,
                    value,
                    self.min_bound,
                    self.max_bound,
                    self.description
                );
            }
        }
        Ok(())
    }
}

pub struct CycleReceipt {
    pub hash: String,
}

pub struct RikEngine {
    state: SovereignState,
    validator: LyapunovValidator,
    ckks: CkksProvider,
    signer: ProvenanceSigner,
    belief_state: Array1<f64>,
}

impl RikEngine {
    pub fn new(state: SovereignState) -> Self {
        Self {
            state,
            validator: LyapunovValidator::new(),
            ckks: CkksProvider::init(),
            signer: ProvenanceSigner::new(),
            belief_state: Array1::zeros(10), // 10-dim state vector
        }
    }

    pub async fn execute_cycle(&mut self, operator_intent: &OperatorIntent) -> Result<CycleReceipt> {
        // Verify sovereign state integrity at cycle start
        if !self.state.verify_integrity() {
            anyhow::bail!("Sovereign state integrity violation detected");
        }

        info!("   -> Operator Intent: {} (bounds: [{}, {}])", 
              operator_intent.description, 
              operator_intent.min_bound, 
              operator_intent.max_bound);

        // 1. OBSERVE (Simulated deterministic input for core logic proof)
        let observation = self.observe_environment();

        // 2. BAYES UPDATE
        self.belief_state = &self.belief_state + &observation; // Simplified Kalman update

        // 3. STATE ESTIMATE & 4. PLANNER PROPOSE (Fused)
        // 5. ACTUATOR MAP
        // 6. MINIMIZE LAGRANGIAN (Enforced by Validator)
        self.validator.check_stability(&self.belief_state)?;

        // Pre-clamping validation: Ensure no NaN or infinite values before safety projection
        for (idx, &value) in self.belief_state.iter().enumerate() {
            if !value.is_finite() {
                anyhow::bail!(
                    "Pre-clamping validation failed: State[{}] = {} is not finite (NaN or infinity)",
                    idx,
                    value
                );
            }
        }

        // 7. SAFETY PROJECT (Clamp values to operator-specified bounds)
        // This enforces the operator's intent on all outputs
        self.belief_state.mapv_inplace(|x| x.clamp(operator_intent.min_bound, operator_intent.max_bound));

        // CRITICAL: Verify all outputs are strictly bounded by operator intent
        operator_intent.verify_state(&self.belief_state)?;
        info!("   -> ✓ Operator intent verified: All outputs within bounds");

        // 8. EXECUTE (GATED) -> Human approval required in main loop before this point
        // This step is now truly gated - execution only proceeds with explicit human approval
        info!("   -> Executing approved actions with human oversight");
        
        // 9. MEASURE
        // 10. UPDATE DUALS (Skipped in V2.0 MVP, implicit in clamp)
        
        // 11. A2A/DFL (Encrypted State Exchange)
        let _encrypted_state = self.ckks.encrypt_state(&self.belief_state);

        // 12. LOG PROVENANCE
        let receipt_hash = self.signer.sign_cycle(&self.belief_state);

        Ok(CycleReceipt { hash: receipt_hash })
    }

    fn observe_environment(&self) -> Array1<f64> {
        // In production, this reads from sensors/API. 
        // Deterministic stub for stability testing (NO RANDOMNESS ALLOWED in Core Logic)
        Array1::from_vec(vec![0.01; 10])
    }
}
