use std::borrow::Borrow;
use std::sync::LazyLock;
use std::path::Path;

use redb::ReadableDatabase;
use redb::ReadableTableMetadata;
use redb::{Database, ReadableTable, TableDefinition};

use crate::man::setting::{self, GlobalSettings};
use crate::util::{Error, Result};
use crate::web::server;

const TABLE_FILE_NAME: &str = "workflow.db";

#[macro_export]
macro_rules! db_executor (
    ($func: expr, $prefix: expr, $suffix: expr, $($bind: expr),*) => ({
        let table_name = format!("{}{}", $prefix, $suffix);
        let table: TableDefinition<&str, &[u8]> = TableDefinition::new(&table_name);
        $func(table $(,($bind))*)
    });
);

pub(crate) static DB: LazyLock<Database> = LazyLock::new(|| {
    let data_folder = Path::new(".").join("data");
    if !data_folder.exists() {
        std::fs::create_dir(&data_folder).expect("Create data directory failed.");
    }
    let path = data_folder.join(TABLE_FILE_NAME);
    if path.exists() {
        Database::open(TABLE_FILE_NAME).expect("Open database failed.")
    } else {
        let db = Database::create(TABLE_FILE_NAME).expect("Create database failed.");
        // let write_txn = db.begin_write().expect("Starting transaction failed");
        // let _ = write_txn.open_table(TABLE).expect("Opening table failed");
        // // let _ = write_txn.open_table(RUNTIME_NODE_TABLE)?;
        // write_txn.commit().expect("Commiting transaction failed");
        db
    }
});

pub(crate) async fn init() -> Result<GlobalSettings> {
    let is_en = *server::IS_EN;

    // Settings
    setting::init_table()?;
    if setting::exists()? {
        return Ok(setting::get_global_settings()?.unwrap());
    }
    let settings = setting::init_global_settings()?;

    context::init()?;
    Ok(settings)
}

pub(crate) fn init_table<K, V>(table: TableDefinition<K, V>) -> Result<()>
where
    K: redb::Key,
    for<'b> V: redb::Value<SelfType<'b> = &'b [u8]>,
{
    let write_txn = DB.begin_write().expect("Starting transaction failed");
    let _ = write_txn.open_table(table).expect("Opening table failed");
    write_txn.commit().expect("Commiting transaction failed");
    Ok(())
}

// https://users.rust-lang.org/t/requesting-help-with-a-lifetime-problem-using-redb/98553
// https://doc.rust-lang.org/nomicon/hrtb.html
// https://users.rust-lang.org/t/implementation-is-not-general-enough/57433/4
pub(crate) fn query<'a, K, V, KEY, D>(table: TableDefinition<K, V>, key: KEY) -> Result<Option<D>>
where
    K: redb::Key,
    for<'b> V: redb::Value<SelfType<'b> = &'b [u8]>,
    D: serde::de::DeserializeOwned,
    KEY: Borrow<&'a str> + std::borrow::Borrow<<K as redb::Value>::SelfType<'a>>,
{
    let read = DB.begin_read()?;
    let table = read.open_table(table)?;
    let r = table.get(key)?;
    if let Some(d) = r {
        let s: D = serde_json::from_slice(d.value())?;
        Ok(Some(s))
    } else {
        Ok(None)
    }
}

pub(crate) fn get_all<K, V, D>(table: TableDefinition<K, V>) -> Result<Vec<D>>
where
    K: redb::Key,
    for<'b> V: redb::Value<SelfType<'b> = &'b [u8]>,
    D: serde::de::DeserializeOwned,
{
    let read = DB.begin_read()?;
    let table = read.open_table(table)?;
    let mut v: Vec<D> = Vec::with_capacity(20);
    for (_key, value) in (table.iter()?).flatten() {
        let s: D = serde_json::from_slice(value.value())?;
        v.push(s)
    }
    Ok(v)
}

pub(crate) fn range<'a, K, V, KR, D>(
    table: TableDefinition<K, V>,
    range: impl std::ops::RangeBounds<KR> + 'a,
) -> Result<Vec<D>>
where
    K: redb::Key,
    for<'b> V: redb::Value<SelfType<'b> = &'b [u8]>,
    KR: Borrow<K::SelfType<'a>> + 'a,
    D: serde::de::DeserializeOwned,
{
    let read = DB.begin_read()?;
    let table = read.open_table(table)?;
    let r = table.range(range)?;
    let mut v: Vec<D> = Vec::with_capacity(10);
    for d in r {
        let (_key, value) = d?;
        let s: D = serde_json::from_slice(value.value())?;
        v.push(s)
    }
    Ok(v)
}

pub(crate) fn count<K, V>(table: TableDefinition<K, V>) -> Result<u64>
where
    K: redb::Key,
    for<'a> V: redb::Value<SelfType<'a> = &'a [u8]>,
{
    let read = DB.begin_read()?;
    let table = read.open_table(table)?;
    let l = table.len()?;
    Ok(l)
}

// key: impl for<'a> Borrow<K::SelfType<'a>>,
// key: K::SelfType<'_>,
// pub(crate) fn write<K, V, D>(table: TableDefinition<K, V>, key: &str, value: &D) -> Result<()>
// https://users.rust-lang.org/t/requesting-help-with-saving-data-into-redb-lifetime-problem/98586/7
pub(crate) fn write<V, D>(table: TableDefinition<&str, V>, key: &str, value: &D) -> Result<()>
where
    V: for<'a> redb::Value<SelfType<'a> = &'a [u8]>,
    D: serde::Serialize,
{
    match serde_json::to_vec(value) {
        Ok(r) => {
            let write_txn = DB.begin_write()?;
            {
                let mut table = write_txn.open_table(table)?;
                // 这里不能用key，是因为insert方法，限定了两个参数是一个生命周期，而r.as_str()短于key的，会编译不通过
                table.insert(key, r.as_slice())?;
            }
            write_txn.commit()?;
            Ok(())
        }
        Err(e) => Err(Error::WithMessage(format!("{e:?}"))),
    }
}

pub(crate) fn remove<'a, K, V, KEY>(table: redb::TableDefinition<K, V>, key: KEY) -> Result<()>
where
    K: redb::Key,
    for<'b> V: redb::Value<SelfType<'b> = &'b [u8]>,
    KEY: Borrow<&'a str> + std::borrow::Borrow<<K as redb::Value>::SelfType<'a>>,
{
    // let db = Database::open(TABLE_FILE_NAME)?;
    let write_txn = DB.begin_write()?;
    {
        let mut table = write_txn.open_table(table)?;
        table.remove(key)?;
    }
    write_txn.commit()?;
    Ok(())
}

pub(crate) fn delete_table<'a, K, V>(table: redb::TableDefinition<K, V>) -> Result<()>
where
    K: redb::Key,
    for<'b> V: redb::Value<SelfType<'b> = &'b [u8]>,
{
    // let db = Database::open(TABLE_FILE_NAME)?;
    let write_txn = DB.begin_write()?;
    {
        write_txn.delete_table(table)?;
    }
    write_txn.commit()?;
    Ok(())
}