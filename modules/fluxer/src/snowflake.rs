// Copyright 2026 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use fluxer_neptunium::model::id::{Id, marker::IdMarker};
use sea_query::{Nullable, SimpleExpr, Value};
use sqlx::{Database, Decode, Encode, Type};
use std::fmt;

#[derive(Decode, Encode, Debug, Clone, Copy)]
pub struct Snowflake(pub i64);

impl fmt::Display for Snowflake {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}

impl<DB: Database> Type<DB> for Snowflake
where i64: sqlx::Type<DB>
{
  #[inline]
  fn type_info() -> DB::TypeInfo {
    i64::type_info()
  }
}

impl Nullable for Snowflake {
  fn null() -> Value {
    <i64 as Nullable>::null()
  }
}

impl Into<SimpleExpr> for Snowflake {
  fn into(self) -> SimpleExpr {
    self.0.into()
  }
}

impl<T: IdMarker> From<Id<T>> for Snowflake {
  #[inline]
  fn from(input: Id<T>) -> Self {
    Self(input.into_inner() as i64)
  }
}

impl<T: IdMarker> Into<Id<T>> for Snowflake {
  #[inline]
  fn into(self) -> Id<T>{
    Id::new(self.0 as u64)
  }
}
