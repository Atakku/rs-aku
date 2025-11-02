// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use aku_core::*;
use sqlx::{PgPool, postgres::PgPoolOptions};

once_cell!(pub pg: PgPool);

use_mod!(
  mod query;
  mod schema;
);

pub async fn init() -> R {
  init_pg(PgPoolOptions::default().connect(&get_env!("DATABASE_URL")).await?);
  Ok(())
}
