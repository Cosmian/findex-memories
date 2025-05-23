// #[cfg(not(any(
//     feature = "redis-mem",
//     feature = "sqlite-mem",
//     feature = "postgres-mem"
// )))]
// panic!(
//     "You must enable at least one database backend feature: redis-mem, sqlite-mem, or postgres-mem.\n\
//     Example (cargo.toml):\n\
//     [dependencies]\n\
//     cosmian_findex_memories = { version = "1.0", features = [\"redis-mem\"] }\n\""
// );

#[cfg(feature = "redis-mem")]
mod redis_mem;
#[cfg(feature = "redis-mem")]
pub use redis_mem::{RedisMemory, RedisMemoryError};

#[cfg(feature = "sqlite-mem")]
mod sqlite_mem;
#[cfg(feature = "sqlite-mem")]
pub use sqlite_mem::{SqliteMemory, SqliteMemoryError, FINDEX_TABLE_NAME};

#[cfg(feature = "postgres-mem")]
mod postgresql_mem;
#[cfg(feature = "postgres-mem")]
pub use postgresql_mem::{PostgresMemory, PostgresMemoryError};
