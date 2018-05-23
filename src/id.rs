use diesel::serialize;
use diesel::serialize::Output;
use diesel::sql_types::Binary;
use diesel::types::ToSql;
use std::io::Write;
use uuid;
use uuid::Uuid;

use diesel::backend::Backend;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::expression::bound::Bound;
use diesel::sql_types::HasSqlType;
use diesel::sqlite::Sqlite;
use juniper::InputValue;
use juniper::Value;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, AsExpression, FromSqlRow, Copy, Clone)]
pub struct UUID(pub Uuid);

impl UUID {
    pub fn parse_str(input: &str) -> Result<UUID, uuid::ParseError> {
        Ok(UUID(Uuid::parse_str(input)?))
    }
}

impl<DB: Backend + HasSqlType<Binary>> ToSql<Binary, DB> for UUID {
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let bytes = self.0.as_bytes();
        <[u8] as ToSql<Binary, DB>>::to_sql(bytes, out)
    }
}

impl FromSql<Binary, Sqlite> for UUID {
    fn from_sql(bytes: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
        let bytes_vec: Vec<u8> = <Vec<u8> as FromSql<Binary, Sqlite>>::from_sql(bytes)?;
        Ok(UUID(Uuid::from_bytes(&bytes_vec)?))
    }
}

impl AsExpression<Binary> for UUID {
    type Expression = Bound<Binary, UUID>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<'a> AsExpression<Binary> for &'a UUID {
    type Expression = Bound<Binary, &'a UUID>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl Hash for UUID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        let inner: Vec<Uuid> = data.iter().map(|s| s.0).collect();
        Uuid::hash_slice(inner.as_ref(), state);
    }
}

impl PartialEq for UUID {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    fn ne(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
    }
}

impl Eq for UUID {}

impl ToString for UUID {
    fn to_string(&self) -> String {
        self.0.simple().to_string()
    }
}

graphql_scalar!(UUID as "ID" {
    resolve(&self) -> Value {
        Value::string(self.to_string())
    }

    from_input_value(v: &InputValue) -> Option<UUID> {
        match *v {
            InputValue::String(ref s) => UUID::parse_str(s).ok(),
            _ => None
        }
    }

});
