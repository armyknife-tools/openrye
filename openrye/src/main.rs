// TODO-TEACHING: Atomics provide thread-safe shared state without locks
// Ordering::Relaxed is sufficient for simple flags
use std::sync::atomic::{AtomicBool, Ordering};

use crate::utils::panic::trap_bad_pipe;
use crate::utils::QuietExit;

// TODO-TEACHING: The #[macro_use] attribute imports all macros from the module
// TODO-IMPROVEMENT: Consider using explicit macro imports for clarity
#[macro_use]
mod tui;

// TODO-INNOVATIVE: AI module for intelligent error recovery and suggestions
mod ai;
mod bootstrap;
mod cli;
mod config;
mod consts;
mod installer;
mod lock;
mod platform;  // TODO-TEACHING: Platform-specific code abstraction
mod pyproject;
// TODO-INNOVATIVE: Rust-Python interop via PyO3 for performance
// mod rust_interop;
mod sources;
mod sync;  // TODO-IMPROVEMENT: Consider async/await for parallel operations
mod utils;
mod uv;

// TODO-TEACHING: Global state using atomics - thread-safe without mutexes
// Static variables have 'static lifetime (entire program duration)
static SHOW_CONTINUE_PROMPT: AtomicBool = AtomicBool::new(false);
static DISABLE_CTRLC_HANDLER: AtomicBool = AtomicBool::new(false);

/// Changes the shutdown behavior to request a continue prompt.
pub fn request_continue_prompt() {
    SHOW_CONTINUE_PROMPT.store(true, Ordering::Relaxed);
}

/// Disables the ctrl-c handler
pub fn disable_ctrlc_handler() {
    DISABLE_CTRLC_HANDLER.store(true, Ordering::Relaxed);
}

// TODO-TEACHING: Entry point - note the lack of Result return type
// Errors are handled internally with exit codes
pub fn main() {
    // TODO-IMPROVEMENT: Add structured logging initialization here
    crate::utils::panic::set_panic_hook();

    // TODO-TEACHING: Graceful shutdown with platform-specific exit codes
    // The 'move' keyword transfers ownership of captured variables into the closure
    ctrlc::set_handler(move || {
        if !DISABLE_CTRLC_HANDLER.load(Ordering::Relaxed) {
            let term = console::Term::stderr();
            term.show_cursor().ok();
            term.flush().ok();
            // TODO-TEACHING: cfg! macro for compile-time platform detection
            std::process::exit(if cfg!(windows) {
                0xC000013Au32 as i32  // Windows-specific CTRL_C exit code
            } else {
                130  // Unix convention: 128 + signal number (SIGINT = 2)
            });
        }
    })
    .unwrap();  // TODO-IMPROVEMENT: Handle error gracefully instead of unwrap

    // TODO-TEACHING: Higher-order function pattern - passing closures
    trap_bad_pipe(|| {
        // TODO-IMPROVEMENT: Add performance monitoring/metrics here
        let result = cli::execute();
        // TODO-TEACHING: Pattern matching for error handling
        let status = match result {
            Ok(()) => 0,
            Err(err) => {
                if let Some(err) = err.downcast_ref::<clap::Error>() {
                    err.print().ok();
                    err.exit_code()
                } else if let Some(QuietExit(code)) = err.downcast_ref() {
                    *code
                } else {
                    error!("{:?}", err);
                    1
                }
            }
        };

        if SHOW_CONTINUE_PROMPT.load(Ordering::Relaxed) {
            echo!("Press any key to continue");
            console::Term::buffered_stderr().read_key().ok();
        }
        status
    });
}
