// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![allow(unused)]

pub use aku_core::*;
#[cfg(feature = "axum")]
pub use aku_axum::*;
#[cfg(feature = "cron")]
pub use aku_cron::*;
#[cfg(feature = "postgres")]
pub use aku_postgres::*;

pub async fn init() -> R {
  #[cfg(feature = "cron")]
  aku_cron::init().await?;
  #[cfg(feature = "postgres")]
  aku_postgres::init().await?;
  Ok(())
}
