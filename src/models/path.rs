use diesel::backend::Backend;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::expression::bound::Bound;
use diesel::serialize;
use diesel::serialize::Output;
use diesel::sql_types::HasSqlType;
use diesel::sql_types::Text;
use diesel::types::ToSql;

use std::io::Write;

use diesel::sqlite::Sqlite;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, AsExpression, FromSqlRow, Clone)]
pub struct PathWrapper(PathBuf);

impl<DB: Backend + HasSqlType<Text>> ToSql<Text, DB> for PathWrapper {
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let text = self.0.to_str().unwrap().to_owned();
        <String as ToSql<Text, DB>>::to_sql(&text, out)
    }
}

impl FromSql<Text, Sqlite> for PathWrapper {
    fn from_sql(bytes: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
        let string: String = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        Ok(PathWrapper(Path::new(&string).to_owned()))
    }
}

impl AsExpression<Text> for PathWrapper {
    type Expression = Bound<Text, PathWrapper>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<'a> AsExpression<Text> for &'a PathWrapper {
    type Expression = Bound<Text, &'a PathWrapper>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<'a> From<&'a Path> for PathWrapper {
    fn from(path: &'a Path) -> Self {
        PathWrapper(path.to_owned())
    }
}
