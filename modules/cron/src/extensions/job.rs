// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use aku_core::*;
use futures::future::BoxFuture;
use tokio_cron_scheduler::Job;

use crate::get_cron;

#[allow(async_fn_in_trait)]
#[extend::ext]
pub impl Job {
  async fn async_from(
    &mut self,
    shed: impl Into<String>,
    run: fn() -> BoxFuture<'static, ()>,
  ) -> R {
    get_cron()?.add(Job::new_async(shed.into(), move |_, _| run())?).await?;
    Ok(())
  }
}
