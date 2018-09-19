use diesel::backend::Backend;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::serialize;
use diesel::serialize::Output;
use diesel::sql_types::Binary;
use diesel::sql_types::HasSqlType;

use std::io::Write;

use diesel::sqlite::Sqlite;
use diesel::types::ToSql;
use std::ffi::OsString;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, AsExpression, FromSqlRow, Clone)]
#[sql_type = "Binary"]
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

impl<DB: Backend + HasSqlType<Binary>> ToSql<Binary, DB> for PathWrapper {
    #[cfg(unix)]
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        use std::os::unix::ffi::OsStrExt;

        let bytes = self.0.as_os_str().as_bytes();
        <[u8] as ToSql<Binary, DB>>::to_sql(bytes, out)
    }
}

impl FromSql<Binary, Sqlite> for PathWrapper {
    #[cfg(unix)]
    fn from_sql(bytes: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
        use std::os::unix::ffi::OsStringExt;

        let raw = <Vec<u8> as FromSql<Binary, Sqlite>>::from_sql(bytes)?;
        let os_string = OsString::from_vec(raw);
        Ok(PathWrapper(Path::new(&os_string).to_owned()))
    }
}

impl From<PathBuf> for PathWrapper {
    fn from(path: PathBuf) -> Self {
        PathWrapper(path)
    }
}

impl<'a> From<&'a Path> for PathWrapper {
    fn from(path: &'a Path) -> Self {
        PathWrapper(path.to_owned())
    }
}

impl Into<PathBuf> for PathWrapper {
    fn into(self) -> PathBuf {
        self.0
    }
}
