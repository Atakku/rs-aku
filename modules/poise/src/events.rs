use aku_core::R;
use poise::serenity_prelude::FullEvent;

use crate::init_ctx;

#[poise::serenity_prelude::async_trait]
pub trait ErrorableEventHandler: Sync + Send + 'static {
  async fn dispatch(
    &self,
    ctx: &poise::serenity_prelude::Context,
    e: &poise::serenity_prelude::FullEvent,
  ) -> R;
}

pub struct InternalEventHandler;

#[poise::serenity_prelude::async_trait]
impl poise::serenity_prelude::EventHandler for InternalEventHandler {
  async fn dispatch(
    &self,
    ctx: &poise::serenity_prelude::Context,
    fe: &poise::serenity_prelude::FullEvent,
  ) {
    if let FullEvent::Ready { .. } = &fe {
      init_ctx(ctx.clone());
    }
    for event in &crate::poise().handlers {
      if let Err(err) = event.dispatch(ctx, fe).await {
        //log::warn!("Error in event {err}");
      }
    }
  }
}
