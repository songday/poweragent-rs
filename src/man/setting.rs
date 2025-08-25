use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::db;
use crate::util::{Error, Result};

use crate::web::server;

#[derive(Deserialize, Serialize)]
pub(crate) struct GlobalSettings {
    pub(crate) ip: String,
    pub(crate) port: u16,
    #[serde(rename = "selectRandomPortWhenConflict")]
    pub(crate) select_random_port_when_conflict: bool,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        GlobalSettings {
            ip: String::from("127.0.0.1"),
            port: 12715,
            select_random_port_when_conflict: true,
        }
    }
}

pub(crate) const TABLE: redb::TableDefinition<&str, &[u8]> = redb::TableDefinition::new("setting");
pub(crate) const GLOBAL_SETTINGS_KEY: &str = "global-settings";

pub(crate) fn init_table() -> Result<()> {
    db::init_table(TABLE)
}

pub(crate) fn exists() -> Result<bool> {
    let cnt = db::count(TABLE)?;
    Ok(cnt > 0)
}

pub(crate) fn init_global_settings() -> Result<GlobalSettings> {
    let global_settings = GlobalSettings::default();
    db::write(TABLE, GLOBAL_SETTINGS_KEY, &global_settings)?;
    let offset = chrono::Utc::now() - chrono::DateTime::UNIX_EPOCH;
    db::write(TABLE, "db_init_time", dbg!(&offset.num_milliseconds()))?;
    db::write(TABLE, "version", &String::from(server::VERSION))?;
    Ok(global_settings)
}

pub(crate) fn get_global_settings() -> Result<Option<GlobalSettings>> {
    db::query(TABLE, GLOBAL_SETTINGS_KEY)
}

pub(crate) fn save_global_settings(data: &GlobalSettings) -> Result<()> {
    let addr = format!("{}:{}", data.ip, data.port);
    let _: SocketAddr = addr.parse().map_err(|_| {
        log::error!("Saving invalid listen IP: {}", &addr);
        Error::WithMessage(String::from("lang.settings.invalidIp"))
    })?;
    db::write(TABLE, GLOBAL_SETTINGS_KEY, &data)
}