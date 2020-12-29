use diesel::backend::Backend;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::serialize;
use diesel::serialize::Output;
use diesel::sql_types::Binary;
use diesel::sql_types::HasSqlType;
use diesel::types::ToSql;
use juniper::InputValue;
use juniper::Value;
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
        let bytes = number_to_arr(value);

        UUID(Uuid::from_bytes(bytes))
    }

    pub fn new() -> UUID {
        UUID(Uuid::new_v4())
    }
}

fn number_to_arr(value: u64) -> [u8; 16] {
    let mut bytes = [0; 16];
    for i in 0..(64 / 8) {
        bytes[16 - 1 - i] = ((value >> (8 * i)) & 0x000000ff) as u8;
    }

    bytes
}

#[cfg(test)]
mod test {
    use super::number_to_arr;

    #[test]
    fn test_number_to_arr_zero() {
        assert_eq!(number_to_arr(0), [0; 16]);
    }

    #[test]
    fn test_number_to_arr_mid() {
        assert_eq!(
            number_to_arr(270),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 14]
        )
    }

    #[test]
    fn test_number_to_arr_max() {
        let arr = number_to_arr(u64::max_value());
        assert_eq!(arr[8..16], [u8::max_value(); 8]);
        assert_eq!(arr[0..8], [0; 8]);
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

impl ToString for UUID {
    fn to_string(&self) -> String {
        self.0.to_simple_ref().to_string()
    }
}

impl Into<UUID> for u64 {
    fn into(self) -> UUID {
        UUID::from_number(self)
    }
}

impl From<Uuid> for UUID {
    fn from(id: Uuid) -> Self {
        UUID(id)
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
