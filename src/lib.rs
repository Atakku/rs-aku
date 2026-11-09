// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![allow(unused)]

use std::sync::{LazyLock, Mutex};

pub use aku_core::*;
#[cfg(feature = "cron")]
pub use aku_cron::*;
#[cfg(all(feature = "fluxer", not(feature = "serenity")))]
pub use aku_fluxer::*;
#[cfg(feature = "postgres")]
pub use aku_postgres::*;
#[cfg(all(feature = "serenity", not(feature = "fluxer")))]
pub use aku_serenity::*;
