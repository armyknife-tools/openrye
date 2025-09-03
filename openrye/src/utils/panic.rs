// TODO-TEACHING: Advanced panic handling with custom hooks
// This module demonstrates Rust's panic infrastructure

use std::any::Any;
use std::{panic, process};

// TODO-TEACHING: Type erasure with Any trait - allows runtime type checking
// dyn keyword indicates trait object (dynamic dispatch)
fn is_bad_pipe(payload: &dyn Any) -> bool {
    // TODO-TEACHING: downcast_ref attempts to convert Any to concrete type
    // is_some_and is a convenient Option method (Rust 1.70+)
    payload
        .downcast_ref::<String>()
        .is_some_and(|x| x.contains("failed printing to stdout: "))
}

/// Registers a panic hook that hides stdout printing failures.
// TODO-IMPROVEMENT: Add configurable panic behavior
pub fn set_panic_hook() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        if !is_bad_pipe(info.payload()) {
            default_hook(info)
        }
    }));
}

/// Catches down panics that are caused by bad pipe errors.
// TODO-TEACHING: Generic function with trait bounds
// F must be: FnOnce (can be called once), Send (thread-safe), Sync (shared between threads)
// The ! return type means this function never returns (diverges)
pub fn trap_bad_pipe<F: FnOnce() -> i32 + Send + Sync>(f: F) -> ! {
    // TODO-TEACHING: catch_unwind allows recovering from panics
    // AssertUnwindSafe is needed because not all types are unwind-safe
    process::exit(match panic::catch_unwind(panic::AssertUnwindSafe(f)) {
        Ok(status) => status,
        Err(panic) => {
            if is_bad_pipe(&panic) {
                1
            } else {
                panic::resume_unwind(panic);
            }
        }
    });
}
