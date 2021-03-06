use diesel::backend::Backend;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::serialize;
use diesel::serialize::Output;
use diesel::sql_types::Binary;
use diesel::sql_types::HasSqlType;
use diesel::types::ToSql;
use juniper::{ParseScalarResult, Value};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Write;
use uuid::Uuid;

#[derive(Debug, AsExpression, FromSqlRow, Copy, Clone, PartialEq, Eq, Hash)]
#[sql_type = "Binary"]
pub struct UUID(Uuid);

impl Default for UUID {
    fn default() -> Self {
        UUID::new()
    }
}

impl UUID {
    pub fn parse_str(input: &str) -> Result<UUID, uuid::Error> {
        Ok(UUID(Uuid::parse_str(input)?))
    }

    pub fn from_number(value: u64) -> UUID {
        UUID(Uuid::from_u128(value as u128))
    }

    pub fn new() -> UUID {
        UUID(Uuid::new_v4())
    }
}

impl<DB: Backend + HasSqlType<Binary>> ToSql<Binary, DB> for UUID {
    fn to_sql<W: Write>(&self, out: &mut Output<'_, W, DB>) -> serialize::Result {
        let bytes = self.0.as_bytes();
        <[u8] as ToSql<Binary, DB>>::to_sql(bytes, out)
    }
}

impl<DB> FromSql<Binary, DB> for UUID
where
    DB: Backend + HasSqlType<Binary>,
    Vec<u8>: FromSql<Binary, DB>,
{
    fn from_sql(bytes: Option<&<DB as Backend>::RawValue>) -> deserialize::Result<Self> {
        let bytes_vec = Vec::<u8>::from_sql(bytes)?;
        Ok(UUID(Uuid::from_slice(&bytes_vec)?))
    }
}

impl Display for UUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.to_simple_ref().fmt(f)
    }
}

impl From<u64> for UUID {
    fn from(num: u64) -> UUID {
        UUID::from_number(num)
    }
}

impl From<Uuid> for UUID {
    fn from(id: Uuid) -> Self {
        UUID(id)
    }
}

// Manually implement the scalar instead of using the inner Uuid impl so that
// `resolve` uses the simple UUID formatter (Uuid uses the default, hyphenated
// format).
#[juniper::graphql_scalar(name = "ID", description = "Uuid")]
impl<S> GraphQLScalar for UUID
where
    S: ScalarValue,
{
    fn resolve(&self) -> Value {
        Value::scalar(self.to_string())
    }

    fn from_input_value(v: &InputValue) -> Option<Self> {
        Uuid::from_input_value(v).map(UUID)
    }

    fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        Uuid::from_str(value)
    }
}
