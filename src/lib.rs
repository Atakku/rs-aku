// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use aku_core::R;

pub async fn init() -> R {
  #[cfg(feature = "cron")]
  aku_cron::init().await?;
  #[cfg(feature = "postgres")]
  aku_postgres::init().await?;
  Ok(())
}