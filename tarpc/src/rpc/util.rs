// Copyright 2018 Google LLC
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash}
};


use std::time::Duration;

#[cfg (not(feature ="wasm"))]
use std::time::{SystemTime};
use tokio::time::Instant as TokioInstant;

#[cfg(feature ="wasm")]
use wasm_timer::{SystemTime, Instant, UNIX_EPOCH};


#[cfg (not(feature ="wasm"))]
use tokio::time::Instant;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;

/// Extension trait for [SystemTimes](SystemTime) in the future, i.e. deadlines.
pub trait TimeUntil {
    /// How much time from now until this time is reached.
    fn time_until(&self) -> Duration;
}

impl TimeUntil for SystemTime {
    fn time_until(&self) -> Duration {
        self.duration_since(SystemTime::now()).unwrap_or_default()
    }
}

pub trait SystemTimeExt { 
    fn into_system_time(self) -> std::time::SystemTime;
}

impl SystemTimeExt for SystemTime { 
    #[cfg(feature ="wasm")]
    fn into_system_time(self) -> std::time::SystemTime { 
        let tmp = self.duration_since(wasm_timer::UNIX_EPOCH).unwrap();
        let sys_time = std::time::UNIX_EPOCH + std::time::Duration::from_secs(tmp.as_secs());
        return sys_time;
    }

    #[cfg (not(feature ="wasm"))]
    fn into_system_time(self) -> std::time::SystemTime { 
        self
    }
}
/*
pub trait InstantExt { 
    fn into_tokio_instant(self) -> tokio::time::Instant;
}

impl InstantExt for Instant { 
    #[cfg(feature ="wasm")]
    fn into_tokio_instant(self) -> tokio::time::Instant { 
        let system_now = SystemTime::now();
        let instant_now = Instant::now();
        let duration = Duration::from_secs(0);
        let secs = system_now.duration_since(UNIX_EPOCH).unwrap();
       

        let std_instant = unsafe { 
            let z = std::sys::wasm::time::Instant::zero().checked_add_duration(&secs).unwrap();
            std::time::Instant(z)
        };
        tokio::time::Instant::from_std(std_instant)
    }

    #[cfg (not(feature ="wasm"))]
    fn into_tokio_instant(self) -> tokio::time::Instant { 
        self
    }
}
*/

/// Collection compaction; configurable `shrink_to_fit`.
pub trait Compact {
    /// Compacts space if the ratio of length : capacity is less than `usage_ratio_threshold`.
    fn compact(&mut self, usage_ratio_threshold: f64);
}

impl<K, V, H> Compact for HashMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    fn compact(&mut self, usage_ratio_threshold: f64) {
        if self.capacity() > 1000 {
            let usage_ratio = self.len() as f64 / self.capacity() as f64;
            if usage_ratio < usage_ratio_threshold {
                self.shrink_to_fit();
            }
        }
    }
}
