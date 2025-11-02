use std::{sync::LazyLock, thread::sleep, time::Duration};

use aku::{R, cron, init_cron};
use tokio::{sync::Mutex, time::Sleep};

#[tokio::main]
async fn main() -> R {
  init_cron().await?;
  cron().start().await?;
  for i in 0..99999 {
    sleep(Duration::from_secs(1));
    println!("meow");
  }
  Ok(())
}

pub fn ident() -> futures::future::BoxFuture<'static, ()> {
  Box::pin(async {
    if let aku::Res::Err(e) = Box::pin(async {
      //$crate::log::info!("Job init: {}", stringify!());

      let mut lock = aku_poise::X.lock().unwrap();
      println!("{}", lock.len());
      sleep(Duration::from_secs((10 - lock.len().min(10)).try_into().unwrap()));
      lock.push("value".to_owned());
      println!("{}", lock.len());
      //$crate::log::info!("Job done: {}", stringify!());
      Ok(())
    })
    .await
    {
      println!("{e}");
      //$crate::log::warn!("Job err: {e}");
    }
  })
}
