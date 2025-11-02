// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

//use std::{collections::HashSet, str::FromStr, sync::{Arc, Mutex}, vec};
//
//use aku_core::*;
//use poise::{BoxFuture, samples::create_application_commands, serenity_prelude::{self, ClientBuilder, FullEvent, GatewayIntents, Token, UserId}};

//pub type Framework = poise::Framework<(), Err>;
//pub type FrameworkContext<'a> = poise::FrameworkContext<'a, (), Err>;
//pub type FrameworkOptions = poise::FrameworkOptions<(), Err>;
//pub type Ctx<'a> = poise::Context<'a, (), Err>;
//pub type AppCtx<'a> = poise::ApplicationContext<'a, (), Err>;
//pub type Command = poise::Command<(), Err>;
//
//once_lock!(|pub poise: Poise| {
//  Poise::default()
//});
//
//pub use poise::serenity_prelude::Context as SCtx;
//once_lock!(pub ctx: SCtx);
//
//use_mod!(
//  mod events;
//);

#[cfg(feature = "postgres")]
use_mod!(
  mod snowflake;
);

//use std::sync::Once;//

//pub static X: std::sync::LazyLock<Mutex<Vec<String>>> = std::sync::LazyLock::new(|| Mutex::new(vec![]));//

//#[derive(Default)]
//pub struct Poise {
//  intents: GatewayIntents,
//  commands: Vec<Command>,
//  handlers: Vec<Box<dyn ErrorableEventHandler>>,
//  options: FrameworkOptions,
//}//
//
//

//  pub async fn start() -> R {
//    //X.get().unwrap().get_mut()?.push("value".into());//

//    X.lock().unwrap().push("value".to_owned());
//    X.lock().unwrap().push("value".to_owned());
//    X.lock().unwrap().push("value".to_owned());//
//

//    //poise().commands.push(register_commands());
//    //let opts = FrameworkOptions {
//    //  commands: poise().commands,
//    //  ..poise().options
//    //};
//    //let fw = Framework::builder().options(opts).build();
//    //ClientBuilder::new(Token::from_str(&get_env!("DISCORD_TOKEN"))?, poise().intents)
//    //.event_handler(events::InternalEventHandler)
//    //  .framework(fw)
//    //  .await?
//    //  .start()
//    //  .await?;
//    Ok(())
//  }//
//

//#[poise::command(prefix_command, hide_in_help, owners_only)]
//pub async fn register_commands(ctx: Ctx<'_>) -> R {
//  ctx.defer().await?;//

//  let cc = create_application_commands(&ctx.framework().options().commands);
//  let count = cc.len();//

//  serenity_prelude::Command::set_global_commands(ctx.http(), &cc).await?;
//  ctx.reply(format!(":gear: Registered {count} commands!")).await?;//

//  Ok(())
//}//
//
//

//  fn owners() -> HashSet<UserId> {
//    let mut owners = HashSet::new();
//    owners.insert(UserId::new(638230362711130132));
//    owners
//  }//

//  fn command_check<'a>(ctx: Ctx<'a>) -> BoxFuture<'a, Res<bool>> {
//    Box::pin(async move {
//      Ok(true)
//    })
//  }
