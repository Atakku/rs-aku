// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use aku_core::*;
use tokio_cron_scheduler::JobScheduler;

once_cell!(pub cron: JobScheduler);

use_mod!(
  mod extensions {
    mod job;
  }
  mod consts;
);

pub async fn init() -> R {
  init_cron(JobScheduler::new().await?);
  Ok(())
}
