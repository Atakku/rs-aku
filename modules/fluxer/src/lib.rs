// Copyright 2026 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use aku_core::*;

#[cfg(feature = "postgres")]
use_mod!(
  mod snowflake;
);
