//! Discrete Optimization utility data-structures

// useful additional warnings (missing docs, crates imported but unused, ...)
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
// #![warn(unsafe_code)]
#![warn(unused_extern_crates)]
#![warn(variant_size_differences)]

// some more useful warnings (no shadowing, similar names, ...)
#![warn(clippy::similar_names)]
#![warn(clippy::print_stdout)]
#![warn(clippy::use_debug)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::shadow_same)]
#![warn(clippy::shadow_reuse)]

// checks integer arithmetic in the project & truncations (useful for debug)
// #![warn(clippy::integer_arithmetic)]
// #![warn(clippy::cast_possible_truncation)]
// #![warn(clippy::cast_possible_wrap)]
// #![warn(clippy::cast_precision_loss)]
// #![warn(clippy::cast_sign_loss)]

/// Implements various priority queues. Including Pareto priority queues.
pub mod priority_queue;

/// defines set data-structures
pub mod set;

/// defines set store data-structures
pub mod set_store;
