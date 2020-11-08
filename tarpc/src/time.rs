#[cfg(not(feature = "wasm"))]
pub use std::time::{Instant, SystemTime};

#[cfg(not(feature = "wasm"))]
pub use tokio::time::{timeout, Elapsed, Timeout};

#[cfg(feature = "wasm")]
pub use wasm_timer::{
    timeout::{timeout, Elapsed, Timeout},
    Instant, SystemTime,
};
