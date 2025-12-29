# Deoxys Hegemony Forge V2.0

**CLASSIFICATION: SOVEREIGN**  
**ROOT AUTHORITY: Alexis Adams (C == XNXAlexis)**

## Overview

Deoxys Core is a deterministic AI control system implementing the 20Hz RIK (Recursive Invariant Kernel) with cryptographic provenance and formal stability guarantees. **The system enforces human-in-the-loop control**, requiring explicit human approval before each cycle execution to prevent autonomous operation.

## Architecture

### Core Components

- **RIK Engine**: 12-step recursive control cycle executing at 20Hz
- **Lyapunov Validator**: Enforces Zero Entropy Law through stability verification
- **CKKS Provider**: Homomorphic encryption for agent-to-agent state exchange
- **Provenance Signer**: Ed25519 cryptographic cycle verification
- **Sovereign Substrate**: Immutable authority root (C == XNXAlexis)

### The 12-Step RIK Cycle

Each cycle requires explicit human approval before execution:

1. **OBSERVE** - Environment state acquisition
2. **BAYES UPDATE** - Belief state refinement
3. **STATE ESTIMATE** - Optimal state inference
4. **PLANNER PROPOSE** - Action candidate generation
5. **ACTUATOR MAP** - Control signal transformation
6. **MINIMIZE LAGRANGIAN** - Constrained optimization
7. **SAFETY PROJECT** - Invariant enforcement
8. **EXECUTE (GATED)** - Controlled actuation **with mandatory human approval**
9. **MEASURE** - Outcome verification
10. **UPDATE DUALS** - Constraint adaptation
11. **A2A/DFL** - Encrypted federated learning
12. **LOG PROVENANCE** - Cryptographic audit trail

## Human-in-the-Loop Control

**No autonomous execution**: The system requires explicit human approval before each cycle. This ensures:
- Human oversight of all AI operations
- Prevention of unintended autonomous behavior
- Compliance with responsible AI principles
- Operator maintains control authority at all times

### Operator Intent Enforcement

**Critical Safety Layer**: Beyond approval, operators specify explicit bounds for all system outputs:
- Operators define minimum and maximum output values for each cycle
- Bounds must be within [-10.0, 10.0] to maintain system stability
- All outputs are strictly clamped to operator-specified bounds during safety projection
- Pre-clamping verification provides audit trail of values before enforcement
- Complete audit trail of operator intentions and clamping operations

This dual-layer control (approval + bounded intent) constrains AI outputs to the operator-defined limits for each cycle, trading fixed, hardcoded safety bounds for operator flexibility. Overall safety and effective determinism therefore depend on the bounds chosen by the operator; the system enforces maximum bounds of ±10.0 to prevent destabilization.

## Zero Entropy Law

The system enforces deterministic behavior through Lyapunov stability:

- Energy function V(x) must remain bounded
- Variance threshold enforcement prevents hallucination
- Fail-stop on invariant violation (no graceful degradation)

## Build & Run

```bash
# Build release binary
cargo build --release

# Execute with human supervision
# The system will prompt for approval before each cycle execution
cargo run --release
```

When running, you will be prompted to specify operator intent and approve each cycle:

### Operator Intent Specification
For each cycle, you must specify:
1. **Min bound**: Minimum allowed value for system outputs (default: -1.0, range: -10.0 to 10.0)
2. **Max bound**: Maximum allowed value for system outputs (default: 1.0, range: -10.0 to 10.0)
3. **Description**: Human-readable description of the intent (default: "Standard bounds")

If invalid bounds are entered, you will be prompted to re-enter them until valid bounds are provided.

### Approval Process
- Enter `y` or `yes` to approve execution with the specified bounds
- Enter `n` or `no` to deny execution
- Enter `exit` after denial to shutdown the system

### Operator Intent Verification
**Critical Safety Feature**: All system outputs are strictly bounded by the operator's specified intent. The system:
- Verifies values are finite (not NaN/infinity) before clamping for audit purposes
- Clamps all state values to the operator-specified bounds during safety projection (Step 7)
- Confirms before execution (Step 8) that, after clamping, every output value lies within the operator-specified bounds
- Treats this pre-execution check as verification of correct clamping rather than as a separate fail-stop on out-of-bounds values
- Logs verification results for audit trails

This ensures that **no output can exceed the bounds explicitly set by the human operator** (within the maximum ±10.0 system stability limits), maintaining strict human control over AI behavior.

## Dependencies

- **Tokio**: Async runtime for 20Hz cycle management
- **ndarray**: State vector mathematics
- **concrete-core**: FHE operations (requires AVX512)
- **ed25519-dalek**: Cryptographic signatures
- **sha2**: Provenance hashing

## License

Proprietary - Axiom Hive © 2025

All rights reserved. This software is proprietary and confidential.
See LICENSE file for full terms.

### Third-Party Dependencies

This software uses open-source libraries that are compatible with proprietary use.
See THIRD_PARTY_NOTICES.md for detailed attribution and license information.

## Contact

Alexis Adams  
sovereign@axiomhive.net  
https://www.axiomhive.net
