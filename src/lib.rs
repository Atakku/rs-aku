// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![allow(unused)]

use std::sync::{LazyLock, Mutex};

pub use aku_core::*;
#[cfg(feature = "axum")]
pub use aku_axum::*;
#[cfg(feature = "cron")]
pub use aku_cron::*;
#[cfg(feature = "postgres")]
pub use aku_postgres::*;
#[cfg(all(feature = "serenity", not(feature = "fluxer")))]
pub use aku_serenity::*;
#[cfg(all(feature = "fluxer", not(feature = "serenity")))]
pub use aku_fluxer::*;

static RT: LazyLock<Mutex<Runtime>> = LazyLock::new(|| Mutex::default());

#[derive(Default)]
pub struct Runtime {

}



#[cfg(feature = "runtime")]
pub async fn runtime() -> R {
  //#[cfg(feature = "cron")]
  //aku_cron::init().await?;
  //#[cfg(feature = "postgres")]
  //aku_postgres::init().await?;
  Ok(())
}
