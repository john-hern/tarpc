
#[cfg (not(feature ="wasm"))]
pub use std::time::{SystemTime, Instant};

#[cfg (not(feature ="wasm"))]
pub use tokio::time::{Timeout, timeout, Elapsed};

#[cfg(feature ="wasm")]
pub use wasm_timer::{SystemTime, Timeout, Instant, timeout, Elapsed };