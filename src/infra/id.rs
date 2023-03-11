use std::marker::PhantomData;

use serde::{Serialize, Serializer};
use sqlx::{Decode, Database};
use tracing::debug;

#[derive(Debug)]
pub struct Id<T>(i64, PhantomData<T>);

impl<T> Id<T> {
    pub fn new(id: i64) -> Self {
        Id(id, PhantomData)
    }
}

impl<T> Id<T> {
    pub fn as_inner(&self) -> &i64 {
        &self.0
    }
}

impl<T> sqlx::Type<sqlx::Postgres> for Id<T> {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <i64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'r, DB: Database, T> sqlx::Decode<'r, DB> for Id<T> 
where i64: Decode<'r, DB>
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <i64 as Decode<DB>>::decode(value)?;
        Ok(Id::<T>::new(value))
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        self.0.serialize(serializer)
    }
}

pub trait IdProvider<T> {
    fn get(&mut self) -> Id<T>;
}

pub struct SnowflakeIdProvider<T> {
    snowflake_id_generator: snowflake::SnowflakeIdGenerator,
    phantom: PhantomData<T>,
}

impl<T> SnowflakeIdProvider<T> {
    pub fn new() -> Self {
        let snowflake_id_generator = snowflake::SnowflakeIdGenerator::new(1, 1);
        let phantom = PhantomData::<T>;

        Self { snowflake_id_generator, phantom }
    }
}

impl<T> IdProvider<T> for SnowflakeIdProvider<T> {
    fn get(&mut self) -> Id<T> {
        let id = self.snowflake_id_generator.real_time_generate(); 
        Id::<T>::new(id)
    }
}
