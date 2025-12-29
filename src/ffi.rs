//! FFI (Foreign Function Interface) module for iOS integration
//! Copyright (c) 2025 Axiom Hive. All rights reserved.
//! 
//! This module exposes a C-compatible API for the Deoxys RIK engine
//! to be called from Swift/Objective-C in iOS applications.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_uint};
use std::ptr;
use std::slice;
use std::sync::{Arc, Mutex};

use crate::rik::RikEngine;
use crate::state::StateVector;

/// Opaque handle to RIK engine instance
pub struct DeoxysHandle {
    engine: Arc<Mutex<RikEngine>>,
}

/// Status codes returned by FFI functions
#[repr(C)]
pub enum DeoxysStatus {
    Success = 0,
    ErrorNullPointer = -1,
    ErrorInvalidHandle = -2,
    ErrorInvariantViolation = -3,
    ErrorInternal = -4,
}

/// Initialize a new Deoxys RIK engine instance
/// 
/// # Safety
/// This function is safe to call from C/Swift
/// 
/// # Parameters
/// - `state_dim`: Dimension of the state vector
/// - `constraint_dim`: Dimension of constraints
/// - `control_dim`: Dimension of control actions
/// 
/// # Returns
/// Opaque handle to the engine, or null on failure
#[no_mangle]
pub extern "C" fn deoxys_init(
    state_dim: c_uint,
    constraint_dim: c_uint,
    control_dim: c_uint,
) -> *mut DeoxysHandle {
    let engine = RikEngine::new(
        state_dim as usize,
        constraint_dim as usize,
        control_dim as usize,
    );
    
    let handle = Box::new(DeoxysHandle {
        engine: Arc::new(Mutex::new(engine)),
    });
    
    Box::into_raw(handle)
}

/// Execute one RIK cycle
/// 
/// # Safety
/// Caller must ensure:
/// - `handle` is a valid pointer returned from `deoxys_init`
/// - `input` points to valid array of length `input_len`
/// - `output` points to valid writable array of length `output_len`
/// 
/// # Parameters
/// - `handle`: Engine handle
/// - `input`: Input state vector
/// - `input_len`: Length of input array
/// - `output`: Output buffer for control action
/// - `output_len`: Length of output buffer
/// 
/// # Returns
/// Status code indicating success or failure
#[no_mangle]
pub extern "C" fn deoxys_step(
    handle: *mut DeoxysHandle,
    input: *const c_double,
    input_len: c_uint,
    output: *mut c_double,
    output_len: c_uint,
) -> c_int {
    if handle.is_null() || input.is_null() || output.is_null() {
        return DeoxysStatus::ErrorNullPointer as c_int;
    }
    
    let handle = unsafe { &*handle };
    
    // Convert C array to Rust Vec
    let input_slice = unsafe { slice::from_raw_parts(input, input_len as usize) };
    let input_vec: Vec<f64> = input_slice.iter().copied().collect();
    let input_state = StateVector::from_vec(input_vec);
    
    // Execute RIK step
    let result = match handle.engine.lock() {
        Ok(engine) => engine.step(&input_state),
        Err(_) => return DeoxysStatus::ErrorInternal as c_int,
    };
    
    match result {
        Ok(action) => {
            // Copy output to C buffer
            let output_slice = unsafe { slice::from_raw_parts_mut(output, output_len as usize) };
            let copy_len = output_slice.len().min(action.u.len());
            for i in 0..copy_len {
                output_slice[i] = action.u[i];
            }
            DeoxysStatus::Success as c_int
        }
        Err(_) => DeoxysStatus::ErrorInvariantViolation as c_int,
    }
}

/// Get current state vector
/// 
/// # Safety
/// Caller must ensure:
/// - `handle` is valid
/// - `output` points to valid writable array of length `output_len`
#[no_mangle]
pub extern "C" fn deoxys_get_state(
    handle: *const DeoxysHandle,
    output: *mut c_double,
    output_len: c_uint,
) -> c_int {
    if handle.is_null() || output.is_null() {
        return DeoxysStatus::ErrorNullPointer as c_int;
    }
    
    let handle = unsafe { &*handle };
    
    let state = match handle.engine.lock() {
        Ok(engine) => engine.get_state(),
        Err(_) => return DeoxysStatus::ErrorInternal as c_int,
    };
    
    let output_slice = unsafe { slice::from_raw_parts_mut(output, output_len as usize) };
    let copy_len = output_slice.len().min(state.x.len());
    for i in 0..copy_len {
        output_slice[i] = state.x[i];
    }
    
    DeoxysStatus::Success as c_int
}

/// Get current cycle count
#[no_mangle]
pub extern "C" fn deoxys_get_cycle_count(handle: *const DeoxysHandle) -> c_uint {
    if handle.is_null() {
        return 0;
    }
    
    let handle = unsafe { &*handle };
    
    match handle.engine.lock() {
        Ok(engine) => engine.get_cycle_count() as c_uint,
        Err(_) => 0,
    }
}

/// Verify cryptographic provenance chain
#[no_mangle]
pub extern "C" fn deoxys_verify_provenance(handle: *const DeoxysHandle) -> c_int {
    if handle.is_null() {
        return DeoxysStatus::ErrorNullPointer as c_int;
    }
    
    let handle = unsafe { &*handle };
    
    match handle.engine.lock() {
        Ok(engine) => {
            if engine.verify_provenance() {
                DeoxysStatus::Success as c_int
            } else {
                DeoxysStatus::ErrorInternal as c_int
            }
        }
        Err(_) => DeoxysStatus::ErrorInternal as c_int,
    }
}

/// Clean up and destroy the engine instance
/// 
/// # Safety
/// - `handle` must be valid pointer from `deoxys_init`
/// - Must not be called more than once with same handle
/// - Handle becomes invalid after this call
#[no_mangle]
pub extern "C" fn deoxys_destroy(handle: *mut DeoxysHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        }
    }
}

/// Get library version string
#[no_mangle]
pub extern "C" fn deoxys_version() -> *const c_char {
    static VERSION_CSTRING: &str = "2.0.0\0";
    VERSION_CSTRING.as_ptr() as *const c_char
}
