// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use aku_core::*;
use tokio_cron_scheduler::{Job, JobScheduler};

once_lock!(|pub cron: JobScheduler| {
  JobScheduler::new().await?
});

pub const EVERY_5_SEC: &str = "*/5 * * * * *";
pub const EVERY_15_SEC: &str = "*/15 * * * * *";
pub const EVERY_30_SEC: &str = "*/30 * * * * *";
pub const EVERY_1_MIN: &str = "0 * * * * *";
pub const EVERY_5_MIN: &str = "0 */5 * * * *";
pub const EVERY_15_MIN: &str = "0 */15 * * * *";
pub const EVERY_30_MIN: &str = "0 */30 * * * *";
pub const EVERY_1_HOUR: &str = "0 0 * * * *";
pub const EVERY_3_HOUR: &str = "0 0 */3 * * *";
pub const EVERY_6_HOUR: &str = "0 0 */6 * * *";
pub const EVERY_12_HOUR: &str = "0 0 */12 * * *";
pub const EVERY_1_DAY: &str = "0 0 0 * * *";

pub async fn add_job<F>(shed: impl Into<String>, run: fn() -> F) -> R
where F: Future<Output = R> + Send + 'static {
  cron()
    .add(Job::new_async(shed.into(), move |_, _| {
      Box::pin(async move {
        if let Res::Err(e) = run().await {
          log::error!("Job error: {e}");
        }
      })
    })?)
    .await?;
  Ok(())
}
