// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use poise::serenity_prelude::all::*;
use sea_query::SimpleExpr;
use sqlx::{Database, Decode, Encode, Type, Value, postgres::PgValue};
use std::fmt;

#[derive(Decode, Encode, Debug)]
pub struct Snowflake(pub(crate) i64);

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

impl Into<SimpleExpr> for Snowflake {
  fn into(self) -> SimpleExpr {
    self.0.into()
  }
}

macro_rules! from_into_snowflake {
  ($ident:ident) => {
    impl From<$ident> for Snowflake {
      #[inline]
      fn from(input: $ident) -> Self {
        Self(input.get() as i64)
      }
    }

    impl Into<$ident> for Snowflake {
      #[inline]
      fn into(self) -> $ident {
        $ident::new(self.0 as u64)
      }
    }
  };
}

from_into_snowflake!(AttachmentId);
from_into_snowflake!(ApplicationId);
from_into_snowflake!(ChannelId);
from_into_snowflake!(GenericChannelId);
from_into_snowflake!(EmojiId);
from_into_snowflake!(GenericId);
from_into_snowflake!(GuildId);
from_into_snowflake!(IntegrationId);
from_into_snowflake!(MessageId);
from_into_snowflake!(RoleId);
from_into_snowflake!(ScheduledEventId);
from_into_snowflake!(StickerId);
from_into_snowflake!(StickerPackId);
from_into_snowflake!(StickerPackBannerId);
from_into_snowflake!(SkuId);
from_into_snowflake!(ThreadId);
from_into_snowflake!(UserId);
from_into_snowflake!(WebhookId);
from_into_snowflake!(AuditLogEntryId);
from_into_snowflake!(InteractionId);
from_into_snowflake!(CommandId);
from_into_snowflake!(CommandPermissionId);
from_into_snowflake!(CommandVersionId);
from_into_snowflake!(TargetId);
from_into_snowflake!(StageInstanceId);
from_into_snowflake!(RuleId);
from_into_snowflake!(ForumTagId);
from_into_snowflake!(EntitlementId);
