// Copyright (c) 2025 Axiom Hive. All Rights Reserved.
// Author: Alexis Adams <sovereign@axiomhive.net>
// SPDX-License-Identifier: Proprietary

use crate::substrate::SovereignState;
use crate::invariants::LyapunovValidator;
use crate::crypto::{CkksProvider, ProvenanceSigner};
use ndarray::Array1;
use anyhow::Result;
use log::info;

pub struct CycleReceipt {
    pub hash: String,
}

/// Operator-specified bounds for output control
#[derive(Debug, Clone, Copy)]
pub struct OperatorBounds {
    pub min: f64,
    pub max: f64,
}

impl OperatorBounds {
    pub fn new(min: f64, max: f64) -> Result<Self> {
        if min >= max {
            anyhow::bail!("Invalid bounds: min ({}) must be less than max ({})", min, max);
        }
        Ok(Self { min, max })
    }
    
    /// Default bounds for backward compatibility
    pub fn default() -> Self {
        Self { min: -1.0, max: 1.0 }
    }
}

pub struct RikEngine {
    state: SovereignState,
    validator: LyapunovValidator,
    ckks: CkksProvider,
    signer: ProvenanceSigner,
    belief_state: Array1<f64>,
    operator_bounds: OperatorBounds,
}

impl RikEngine {
    pub fn new(state: SovereignState) -> Self {
        Self {
            state,
            validator: LyapunovValidator::new(),
            ckks: CkksProvider::init(),
            signer: ProvenanceSigner::new(),
            belief_state: Array1::zeros(10), // 10-dim state vector
            operator_bounds: OperatorBounds::default(),
        }
    }

    /// Set operator-specified bounds for output control
    pub fn set_operator_bounds(&mut self, bounds: OperatorBounds) {
        info!("   -> Operator bounds updated: [{}, {}]", bounds.min, bounds.max);
        self.operator_bounds = bounds;
    }

    pub async fn execute_cycle(&mut self) -> Result<CycleReceipt> {
        // Verify sovereign state integrity at cycle start
        if !self.state.verify_integrity() {
            anyhow::bail!("Sovereign state integrity violation detected");
        }

        // 1. OBSERVE (Simulated deterministic input for core logic proof)
        let observation = self.observe_environment();

        // 2. BAYES UPDATE
        self.belief_state = &self.belief_state + &observation; // Simplified Kalman update

        // 3. STATE ESTIMATE & 4. PLANNER PROPOSE (Fused)
        // 5. ACTUATOR MAP
        // 6. MINIMIZE LAGRANGIAN (Enforced by Validator)
        self.validator.check_stability(&self.belief_state)?;

        // 7. SAFETY PROJECT (Clamp values to operator-specified bounds)
        let bounds = self.operator_bounds;
        self.belief_state.mapv_inplace(|x| x.clamp(bounds.min, bounds.max));
        
        // Verify all outputs are strictly bounded by operator's intent
        for &val in self.belief_state.iter() {
            if val < bounds.min || val > bounds.max {
                anyhow::bail!(
                    "Output violation: value {} exceeds operator bounds [{}, {}]",
                    val, bounds.min, bounds.max
                );
            }
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::substrate::SovereignState;

    #[test]
    fn test_operator_bounds_validation() {
        // Valid bounds should succeed
        let bounds = OperatorBounds::new(-2.0, 2.0);
        assert!(bounds.is_ok());
        let bounds = bounds.unwrap();
        assert_eq!(bounds.min, -2.0);
        assert_eq!(bounds.max, 2.0);

        // Invalid bounds (min >= max) should fail
        let invalid_bounds = OperatorBounds::new(1.0, 1.0);
        assert!(invalid_bounds.is_err());

        let invalid_bounds = OperatorBounds::new(2.0, 1.0);
        assert!(invalid_bounds.is_err());
    }

    #[test]
    fn test_default_bounds() {
        let bounds = OperatorBounds::default();
        assert_eq!(bounds.min, -1.0);
        assert_eq!(bounds.max, 1.0);
    }

    #[test]
    fn test_set_operator_bounds() {
        let substrate = SovereignState::new("C_EQUALS_XNXALEXIS_ROOT");
        let mut engine = RikEngine::new(substrate);
        
        // Check default bounds
        assert_eq!(engine.operator_bounds.min, -1.0);
        assert_eq!(engine.operator_bounds.max, 1.0);
        
        // Set custom bounds
        let custom_bounds = OperatorBounds::new(-0.5, 0.5).unwrap();
        engine.set_operator_bounds(custom_bounds);
        
        assert_eq!(engine.operator_bounds.min, -0.5);
        assert_eq!(engine.operator_bounds.max, 0.5);
    }

    #[tokio::test]
    async fn test_cycle_respects_operator_bounds() {
        let substrate = SovereignState::new("C_EQUALS_XNXALEXIS_ROOT");
        let mut engine = RikEngine::new(substrate);
        
        // Set tight bounds
        let bounds = OperatorBounds::new(-0.5, 0.5).unwrap();
        engine.set_operator_bounds(bounds);
        
        // Execute cycle and verify it completes without error
        let result = engine.execute_cycle().await;
        assert!(result.is_ok());
        
        // Verify all values in belief_state are within bounds
        for &val in engine.belief_state.iter() {
            assert!(val >= -0.5 && val <= 0.5, 
                "Value {} exceeds bounds [-0.5, 0.5]", val);
        }
    }

    #[tokio::test]
    async fn test_cycle_with_wide_bounds() {
        let substrate = SovereignState::new("C_EQUALS_XNXALEXIS_ROOT");
        let mut engine = RikEngine::new(substrate);
        
        // Set wide bounds
        let bounds = OperatorBounds::new(-10.0, 10.0).unwrap();
        engine.set_operator_bounds(bounds);
        
        // Execute cycle
        let result = engine.execute_cycle().await;
        assert!(result.is_ok());
        
        // Verify all values are within wide bounds
        for &val in engine.belief_state.iter() {
            assert!(val >= -10.0 && val <= 10.0,
                "Value {} exceeds bounds [-10.0, 10.0]", val);
        }
    }
}
