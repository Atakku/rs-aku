// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use sqlx::{PgPool, postgres::PgPoolOptions};

once_lock!(|pub pg: PgPool| {
  PgPoolOptions::default().connect(&get_env!("DATABASE_URL")).await?
});

use_mod!(
  mod query;
  mod schema;
);

pub use aku_core::*;
