use axum::Json;
use axum::response::IntoResponse;

use crate::db;
use crate::db_executor;
use crate::util::Result;
use crate::web::server::to_res;
use super::dto::Workflow;

pub(crate) const TABLE_SUFFIX: &str = "workflows";

pub(crate) fn init() -> Result<()> {
    // let table_name = format!("{}-mainflows", robot_id);
    // let main_flow_table: TableDefinition<&str, &[u8]> = TableDefinition::new(&table_name);
    // db::init_table(main_flow_table)?;
    db_executor!(db::init_table, "", TABLE_SUFFIX,)?;
    // let table_name = format!("{}-subflows", robot_id);
    // let sub_flow_table: TableDefinition<&str, &[u8]> = TableDefinition::new(&table_name);
    // db::init_table(sub_flow_table)?;
    Ok(())
}

pub(crate) async fn list(
) -> impl IntoResponse {
    to_res(Ok(()))
}

pub(crate) async fn save(
    Json(data): Json<Workflow>,
) -> impl IntoResponse {
    to_res(Ok(()))
}

pub(crate) async fn delete(
) -> impl IntoResponse {
    to_res(Ok(()))
}