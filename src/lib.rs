// TODO: check these
#![warn(clippy::all, clippy::nursery, clippy::cargo)]
// This is necessary since CryptoCore depends on pkcs8 which depends on an old
// version of rand_core, which depends on an old version of getrandom (0.2.15),
// while CryptoCore also depends on gensym which depends on uuid, which depends
// on a newer version of getrandom (0.3.2).
#![allow(clippy::multiple_crate_versions)]

#[cfg(feature = "redis-mem")]
pub use memory::{RedisMemory, RedisMemoryError};

#[cfg(feature = "sqlite-mem")]
pub use memory::{SqliteMemory, SqliteMemoryError, FINDEX_TABLE_NAME};

#[cfg(feature = "postgres-mem")]
pub use memory::{PostgresMemory, PostgresMemoryError};
