use diesel::backend::Backend;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::serialize;
use diesel::serialize::Output;
use diesel::sql_types::HasSqlType;
use diesel::sql_types::Text;

use std::io::Write;

use diesel::sqlite::Sqlite;
use diesel::types::ToSql;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, AsExpression, FromSqlRow, Clone)]
#[sql_type = "Text"]
pub struct PathWrapper(PathBuf);

impl PathWrapper {
    pub fn as_path(&self) -> &Path {
        self.0.as_ref()
    }
}

impl Deref for PathWrapper {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

impl<T> From<T> for PathWrapper
where
    T: AsRef<Path>,
{
    fn from(path: T) -> Self {
        let path = path.as_ref();
        PathWrapper(path.to_owned())
    }
}
